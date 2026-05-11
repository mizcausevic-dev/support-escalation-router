use crate::data::sample_tickets;
use crate::models::{DashboardSummary, RoutingAnalysis, RoutingRequest, TicketThread};

pub fn dashboard_summary() -> DashboardSummary {
    let tickets = sample_tickets();
    let escalated_threads = tickets
        .iter()
        .filter(|ticket| analyze(ticket_to_request(ticket)).status == "escalate")
        .count();
    let critical_sla_threads = tickets
        .iter()
        .filter(|ticket| ticket.sla_minutes_remaining <= 30)
        .count();

    DashboardSummary {
        service: "support-escalation-router".into(),
        open_tickets: tickets.len(),
        escalated_threads,
        critical_sla_threads,
        dominant_risk: "SLA collapse across enterprise callbacks".into(),
        busiest_lane: "billing-platform".into(),
    }
}

pub fn tickets() -> Vec<TicketThread> {
    sample_tickets()
}

pub fn ticket(id: &str) -> Option<TicketThread> {
    sample_tickets().into_iter().find(|ticket| ticket.id == id)
}

pub fn sample_analysis() -> RoutingAnalysis {
    let ticket = sample_tickets()
        .into_iter()
        .next()
        .expect("sample ticket should exist");
    analyze(ticket_to_request(&ticket))
}

pub fn analyze(request: RoutingRequest) -> RoutingAnalysis {
    let mut score = 12;
    let severity = request.severity.to_lowercase();
    let customer_tier = request.customer_tier.to_lowercase();
    let callback_risk = request.callback_risk.to_lowercase();

    score += match severity.as_str() {
        "critical" => 34,
        "high" => 24,
        "medium" => 14,
        _ => 8,
    };

    if request.sla_minutes_remaining <= 30 {
        score += 26;
    } else if request.sla_minutes_remaining <= 60 {
        score += 15;
    }

    score += request.unresolved_dependencies * 6;
    score += request.handoff_count * 4;

    if customer_tier == "enterprise" || customer_tier == "premier" {
        score += 10;
    }

    if callback_risk == "high" {
        score += 10;
    } else if callback_risk == "moderate" {
        score += 5;
    }

    let status = if score >= 72 {
        "escalate"
    } else if score >= 48 {
        "watch"
    } else {
        "stable"
    };

    let recommended_lane = if request.unresolved_dependencies >= 3 {
        "incident-command"
    } else if request.owner_lane.contains("identity") {
        "identity-systems"
    } else {
        request.owner_lane.as_str()
    };

    let immediate_action = if status == "escalate" {
        "Assign a single escalation owner, freeze secondary callbacks, and route the thread into the highest-accountability lane now."
    } else if status == "watch" {
        "Keep the current owner lane, but collapse duplicate notes and prepare a named escalation path before the next SLA checkpoint."
    } else {
        "Continue within the active queue and monitor for new dependency drag."
    };

    let routing_reason = if status == "escalate" {
        "The ticket is carrying too much SLA and handoff pressure to stay inside a normal support queue."
    } else if status == "watch" {
        "The thread is still controllable, but the handoff pattern is starting to introduce avoidable risk."
    } else {
        "The thread is contained enough to stay with the current lane."
    };

    let mut risks = vec![];
    if request.sla_minutes_remaining <= 30 {
        risks.push("The SLA window is approaching immediate breach.".into());
    }
    if request.handoff_count >= 3 {
        risks.push("Ownership is diffusing across too many handoffs.".into());
    }
    if request.unresolved_dependencies >= 2 {
        risks.push("Dependency drag is slowing a clean resolution path.".into());
    }
    if callback_risk == "high" {
        risks.push("Customer callback confidence is already under strain.".into());
    }

    let mut stabilizers = vec![];
    if !request.next_steps.is_empty() {
        stabilizers.push("A next-step sequence already exists for the owner lane.".into());
    }
    if request.owner_lane.contains("systems") || request.owner_lane.contains("platform") {
        stabilizers.push("The thread already has a specialist lane attached.".into());
    }
    if request.unresolved_dependencies <= 1 {
        stabilizers.push("Dependency drag is still low enough to recover quickly.".into());
    }

    RoutingAnalysis {
        status: status.into(),
        score,
        recommended_lane: recommended_lane.into(),
        routing_reason: routing_reason.into(),
        immediate_action: immediate_action.into(),
        risks,
        stabilizers,
    }
}

fn ticket_to_request(ticket: &TicketThread) -> RoutingRequest {
    RoutingRequest {
        id: ticket.id.clone(),
        title: ticket.title.clone(),
        queue: ticket.queue.clone(),
        severity: ticket.severity.clone(),
        customer_tier: ticket.customer_tier.clone(),
        region: ticket.region.clone(),
        sla_minutes_remaining: ticket.sla_minutes_remaining,
        unresolved_dependencies: ticket.unresolved_dependencies,
        owner_lane: ticket.owner_lane.clone(),
        handoff_count: ticket.handoff_count,
        callback_risk: ticket.callback_risk.clone(),
        blockers: ticket.blockers.clone(),
        next_steps: ticket.next_steps.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn critical_threads_escalate() {
        let analysis = analyze(RoutingRequest {
            id: "test-1".into(),
            title: "Critical".into(),
            queue: "enterprise".into(),
            severity: "critical".into(),
            customer_tier: "enterprise".into(),
            region: "us-east".into(),
            sla_minutes_remaining: 12,
            unresolved_dependencies: 3,
            owner_lane: "billing-platform".into(),
            handoff_count: 4,
            callback_risk: "high".into(),
            blockers: vec!["freeze".into()],
            next_steps: vec!["route".into()],
        });

        assert_eq!(analysis.status, "escalate");
        assert_eq!(analysis.recommended_lane, "incident-command");
    }

    #[test]
    fn low_pressure_threads_stay_stable() {
        let analysis = analyze(RoutingRequest {
            id: "test-2".into(),
            title: "Routine".into(),
            queue: "growth".into(),
            severity: "medium".into(),
            customer_tier: "commercial".into(),
            region: "global".into(),
            sla_minutes_remaining: 180,
            unresolved_dependencies: 0,
            owner_lane: "growth-systems".into(),
            handoff_count: 1,
            callback_risk: "low".into(),
            blockers: vec![],
            next_steps: vec!["respond".into()],
        });

        assert_eq!(analysis.status, "stable");
    }
}
