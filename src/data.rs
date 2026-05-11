use crate::models::TicketThread;

pub fn sample_tickets() -> Vec<TicketThread> {
    vec![
        TicketThread {
            id: "sup-9104".into(),
            title: "Enterprise billing export is timing out before finance close".into(),
            queue: "enterprise-support".into(),
            severity: "critical".into(),
            customer_tier: "enterprise".into(),
            region: "us-east".into(),
            sla_minutes_remaining: 18,
            unresolved_dependencies: 3,
            owner_lane: "billing-platform".into(),
            handoff_count: 4,
            callback_risk: "high".into(),
            blockers: vec![
                "Data export retries are crossing the invoice freeze window".into(),
                "Finance analyst is waiting on a named escalation owner".into(),
            ],
            next_steps: vec![
                "Route to billing-platform and incident command immediately".into(),
                "Freeze nonessential account sync jobs until the export lane clears".into(),
            ],
        },
        TicketThread {
            id: "sup-9108".into(),
            title: "Premier customer cannot complete SSO handoff after regional failover".into(),
            queue: "identity-support".into(),
            severity: "high".into(),
            customer_tier: "premier".into(),
            region: "eu-west".into(),
            sla_minutes_remaining: 42,
            unresolved_dependencies: 2,
            owner_lane: "identity-systems".into(),
            handoff_count: 3,
            callback_risk: "moderate".into(),
            blockers: vec![
                "Failover metadata is still pinned to the old region".into(),
                "Support notes are split across two lanes".into(),
            ],
            next_steps: vec![
                "Consolidate the handoff into identity-systems ownership".into(),
                "Hold customer callback until region pinning is corrected".into(),
            ],
        },
        TicketThread {
            id: "sup-9113".into(),
            title: "Growth support queue is seeing burst volume from launch-day trial routing".into(),
            queue: "growth-operations".into(),
            severity: "high".into(),
            customer_tier: "commercial".into(),
            region: "global".into(),
            sla_minutes_remaining: 64,
            unresolved_dependencies: 1,
            owner_lane: "growth-systems".into(),
            handoff_count: 2,
            callback_risk: "moderate".into(),
            blockers: vec![
                "Trial routing changed without updating support playbooks".into(),
            ],
            next_steps: vec![
                "Shift top-volume issues to growth-systems".into(),
                "Update macros before the next callback wave hits".into(),
            ],
        },
    ]
}
