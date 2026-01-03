use once_cell::sync::Lazy;
use prometheus::{Encoder, IntCounter, IntGauge, Registry, TextEncoder};

static REGISTRY: Lazy<Registry> = Lazy::new(Registry::new);
static RPC_SUCCESS: Lazy<IntCounter> =
    Lazy::new(|| IntCounter::new("fenrir_rpc_success_total", "RPC success").unwrap());
static RPC_FAILURE: Lazy<IntCounter> =
    Lazy::new(|| IntCounter::new("fenrir_rpc_failure_total", "RPC failure").unwrap());
static HEALTH_STATUS: Lazy<IntGauge> =
    Lazy::new(|| IntGauge::new("fenrir_health_status", "Health status").unwrap());

/// Inicializa o registry.
pub fn init_metrics() {
    let _ = REGISTRY.register(Box::new(RPC_SUCCESS.clone()));
    let _ = REGISTRY.register(Box::new(RPC_FAILURE.clone()));
    let _ = REGISTRY.register(Box::new(HEALTH_STATUS.clone()));
}

/// Marca sucesso de RPC.
pub fn record_rpc_success() {
    RPC_SUCCESS.inc();
}

/// Marca falha de RPC.
pub fn record_rpc_failure() {
    RPC_FAILURE.inc();
}

/// Atualiza status de health.
pub fn set_health_status(ok: bool) {
    HEALTH_STATUS.set(if ok { 1 } else { 0 });
}

/// Exporta métricas em texto.
pub fn export_metrics() -> String {
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap_or(());
    String::from_utf8_lossy(&buffer).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metrics_export() {
        init_metrics();
        record_rpc_success();
        set_health_status(true);
        let text = export_metrics();
        assert!(text.contains("fenrir_rpc_success_total"));
        assert!(text.contains("fenrir_health_status"));
    }
}
