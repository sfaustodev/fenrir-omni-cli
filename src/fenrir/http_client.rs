use std::time::Duration;

use once_cell::sync::Lazy;
use reqwest::Client;

static SHARED_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .pool_max_idle_per_host(20)
        .pool_idle_timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(30))
        .build()
        .expect("http client")
});

/// Cliente HTTP com pooling configurado.
pub fn shared_client() -> &'static Client {
    &SHARED_CLIENT
}
