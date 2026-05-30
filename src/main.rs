// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{extract::State, response::Html, routing::get, Json, Router};
use sensor_health_drift_monitor::{
    core::sample_payload,
    render::{render_docs, render_drift_findings, render_overview, render_sensor_lane, render_verification},
};
use std::{net::SocketAddr, sync::Arc};

#[derive(Clone)]
struct AppState(Arc<sensor_health_drift_monitor::core::Payload>);

#[tokio::main]
async fn main() {
    let state = AppState(Arc::new(sample_payload()));
    let app = Router::new()
        .route("/", get(|| async { Html(render_overview()) }))
        .route("/sensor-lane", get(|| async { Html(render_sensor_lane()) }))
        .route("/drift-findings", get(|| async { Html(render_drift_findings()) }))
        .route("/verification", get(|| async { Html(render_verification()) }))
        .route("/docs", get(|| async { Html(render_docs()) }))
        .route("/api/dashboard/summary", get(summary))
        .route("/api/sensor-lane", get(sensor_lane))
        .route("/api/drift-findings", get(drift_findings))
        .route("/api/verification", get(verification))
        .route("/api/sample", get(sample))
        .with_state(state);

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").ok().and_then(|s| s.parse::<u16>().ok()).unwrap_or(5532);
    let addr: SocketAddr = format!("{host}:{port}").parse().expect("valid bind address");
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");
    axum::serve(listener, app).await.expect("server");
}

async fn summary(State(state): State<AppState>) -> Json<sensor_health_drift_monitor::core::Summary> {
    Json(state.0.summary.clone())
}

async fn sensor_lane(State(state): State<AppState>) -> Json<Vec<sensor_health_drift_monitor::core::SensorLaneRecord>> {
    Json(state.0.sensor_lane.clone())
}

async fn drift_findings(State(state): State<AppState>) -> Json<Vec<sensor_health_drift_monitor::core::DriftFinding>> {
    Json(state.0.drift_findings.clone())
}

async fn verification(State(state): State<AppState>) -> Json<Vec<sensor_health_drift_monitor::core::VerificationGate>> {
    Json(state.0.verification.clone())
}

async fn sample(State(state): State<AppState>) -> Json<sensor_health_drift_monitor::core::Payload> {
    Json((*state.0).clone())
}
