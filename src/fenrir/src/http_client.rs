// --- HTTP CLIENT POOLING MODULE ---
// Shared HTTP clients for connection reuse and performance

use once_cell::sync::Lazy;
use reqwest::Client;
use std::time::Duration;

/// Global shared HTTP client with connection pooling
/// Reuses connections across all API calls for better performance
pub static SHARED_HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .pool_max_idle_per_host(10) // Keep 10 idle connections per host
        .pool_idle_timeout(Duration::from_secs(30)) // Close idle connections after 30s
        .timeout(Duration::from_secs(30)) // Default timeout
        .user_agent("Fenrir-CLI/1.0")
        .build()
        .expect("Failed to build shared HTTP client")
});

/// Get a reference to the shared HTTP client
/// This ensures all HTTP requests reuse connections
pub fn get_shared_client() -> &'static Client {
    &SHARED_HTTP_CLIENT
}

/// Create a custom client with specific configuration if needed
/// For specialized use cases that require different settings
pub fn create_custom_client(timeout_secs: u64, max_connections: usize) -> Client {
    Client::builder()
        .pool_max_idle_per_host(max_connections)
        .pool_idle_timeout(Duration::from_secs(30))
        .timeout(Duration::from_secs(timeout_secs))
        .user_agent("Fenrir-CLI/1.0")
        .build()
        .expect("Failed to build custom HTTP client")
}