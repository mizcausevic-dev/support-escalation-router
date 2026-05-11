# Support Escalation Router Architecture

Support Escalation Router is designed as a lightweight Rust control layer that sits between ticket intake and human escalation ownership. The key idea is simple: support pressure should be routable, measurable, and explicitly owned.

## Core Flow

1. A support thread arrives with SLA context, customer tier, and handoff history.
2. The Axum route layer validates and forwards the payload to the escalation engine.
3. The engine scores severity, SLA compression, dependency drag, handoff count, and callback risk.
4. The result becomes a routing decision:
   - `stable`
   - `watch`
   - `escalate`
5. The service returns a recommended owner lane and immediate action.

## Why Rust Here

- the service is small, fast, and infrastructure-flavored
- the domain benefits from predictable request handling and simple deployability
- it broadens the portfolio beyond TypeScript-heavy APIs while still feeling production-minded

## Main Entities

- `TicketThread`
- `RoutingRequest`
- `RoutingAnalysis`
- `DashboardSummary`

## Operational Framing

This repo is intentionally not a generic helpdesk app. It is modeled more like a support control-plane service:

- ticket queue pressure is a routing signal
- owner lanes are explicit
- handoffs are treated as risk, not neutral movement
- customer callback confidence affects the decision

## Endpoints

- `GET /`
- `GET /docs`
- `GET /api/dashboard/summary`
- `GET /api/tickets`
- `GET /api/tickets/{id}`
- `GET /api/sample`
- `POST /api/analyze/route`
