mod data;
mod engine;
mod models;

use axum::{
    extract::Path,
    http::{header::CONTENT_TYPE, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use models::{RoutingRequest, TicketCollection};
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    let port = env::var("PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(4384);

    let app = app();
    let address = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = match tokio::net::TcpListener::bind(address).await {
        Ok(listener) => listener,
        Err(_) => {
            eprintln!(
                "Support Escalation Router could not start because port {} is already in use.",
                port
            );
            eprintln!("Set a different port before running again, for example:");
            eprintln!("$env:PORT = \"4390\"");
            eprintln!("cargo run");
            std::process::exit(1);
        }
    };

    println!("Support Escalation Router listening on http://127.0.0.1:{port}/");
    axum::serve(listener, app).await.unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/docs", get(docs))
        .route("/api/dashboard/summary", get(summary))
        .route("/api/tickets", get(tickets))
        .route("/api/tickets/{id}", get(ticket))
        .route("/api/sample", get(sample))
        .route("/api/analyze/route", post(analyze))
}

async fn root() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "service": "support-escalation-router",
        "language": "Rust",
        "framework": "Axum",
        "description": "Queue-aware escalation routing for support, SLA pressure, and owner handoff planning.",
        "endpoints": [
            "/docs",
            "/api/dashboard/summary",
            "/api/tickets",
            "/api/tickets/{id}",
            "/api/sample",
            "/api/analyze/route"
        ]
    }))
}

async fn docs() -> impl IntoResponse {
    (
        [(CONTENT_TYPE, "text/html; charset=utf-8")],
        Html(
            r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Support Escalation Router Docs</title>
    <style>
      body { font-family: "Segoe UI", sans-serif; background:#0b1320; color:#f3efe1; margin:0; padding:32px; }
      .shell { max-width:960px; margin:0 auto; background:#131d30; border:1px solid #294164; border-radius:20px; padding:28px; }
      h1 { margin:0 0 8px; font-size:40px; line-height:1.08; }
      p, li, code { color:#c6d0e2; }
      code { background:#0d1728; padding:2px 6px; border-radius:6px; }
    </style>
  </head>
  <body>
    <div class="shell">
      <p style="letter-spacing:0.25em;text-transform:uppercase;color:#86c4ff;">Support Escalation Router</p>
      <h1>Rust control layer for SLA pressure, queue ownership, and escalation routing.</h1>
      <p>This service turns support queue volatility into named ownership, route pressure, and immediate next-step guidance.</p>
      <ul>
        <li><code>GET /api/dashboard/summary</code> returns queue posture.</li>
        <li><code>GET /api/tickets</code> returns modeled support threads.</li>
        <li><code>GET /api/sample</code> returns a sample route analysis.</li>
        <li><code>POST /api/analyze/route</code> scores a payload and returns the next action.</li>
      </ul>
    </div>
  </body>
</html>"#,
        ),
    )
}

async fn summary() -> Json<models::DashboardSummary> {
    Json(engine::dashboard_summary())
}

async fn tickets() -> Json<TicketCollection> {
    Json(TicketCollection {
        tickets: engine::tickets(),
    })
}

async fn ticket(Path(id): Path<String>) -> impl IntoResponse {
    match engine::ticket(&id) {
        Some(ticket) => (StatusCode::OK, Json(ticket)).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "ticket_not_found",
                "id": id
            })),
        )
            .into_response(),
    }
}

async fn sample() -> Json<models::RoutingAnalysis> {
    Json(engine::sample_analysis())
}

async fn analyze(Json(payload): Json<RoutingRequest>) -> Json<models::RoutingAnalysis> {
    Json(engine::analyze(payload))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn root_route_returns_ok() {
        let response = app()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn missing_ticket_returns_404() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/api/tickets/does-not-exist")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
