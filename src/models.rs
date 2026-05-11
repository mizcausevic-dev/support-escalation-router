use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TicketThread {
    pub id: String,
    pub title: String,
    pub queue: String,
    pub severity: String,
    pub customer_tier: String,
    pub region: String,
    pub sla_minutes_remaining: i32,
    pub unresolved_dependencies: i32,
    pub owner_lane: String,
    pub handoff_count: i32,
    pub callback_risk: String,
    pub blockers: Vec<String>,
    pub next_steps: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DashboardSummary {
    pub service: String,
    pub open_tickets: usize,
    pub escalated_threads: usize,
    pub critical_sla_threads: usize,
    pub dominant_risk: String,
    pub busiest_lane: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TicketCollection {
    pub tickets: Vec<TicketThread>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoutingRequest {
    pub id: String,
    pub title: String,
    pub queue: String,
    pub severity: String,
    pub customer_tier: String,
    pub region: String,
    pub sla_minutes_remaining: i32,
    pub unresolved_dependencies: i32,
    pub owner_lane: String,
    pub handoff_count: i32,
    pub callback_risk: String,
    pub blockers: Vec<String>,
    pub next_steps: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoutingAnalysis {
    pub status: String,
    pub score: i32,
    pub recommended_lane: String,
    pub routing_reason: String,
    pub immediate_action: String,
    pub risks: Vec<String>,
    pub stabilizers: Vec<String>,
}
