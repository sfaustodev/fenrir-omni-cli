use axum::{response::Html, routing::get, Json, Router};
use serde::Serialize;
use std::net::SocketAddr;

use crate::metrics;

/// Relatório de saúde.
#[derive(Serialize)]
pub struct HealthReport {
    pub status: String,
    pub uptime_seconds: u64,
}

static START: once_cell::sync::Lazy<std::time::Instant> =
    once_cell::sync::Lazy::new(std::time::Instant::now);

/// Executa health check básico.
pub fn check() -> HealthReport {
    let uptime = START.elapsed().as_secs();
    let ok = uptime > 0;
    metrics::set_health_status(ok);
    HealthReport {
        status: if ok { "ok" } else { "degraded" }.to_string(),
        uptime_seconds: uptime,
    }
}

async fn health_handler() -> Json<HealthReport> {
    Json(check())
}

async fn status_handler() -> Html<String> {
    let report = check();
    let html = format!(
        "<html><body><h1>🐺 FENRIR STATUS</h1><p>Status: {}</p><p>Uptime: {}s</p></body></html>",
        report.status, report.uptime_seconds
    );
    Html(html)
}

async fn metrics_handler() -> String {
    metrics::export_metrics()
}

/// Serve health, metrics e status page.
pub async fn serve(addr: SocketAddr) -> anyhow::Result<()> {
    metrics::init_metrics();
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/status", get(status_handler))
        .route("/metrics", get(metrics_handler));
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
    Ok(())
}
