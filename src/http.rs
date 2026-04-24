use reqwest::{Client, Method};
use crate::config::Config;

pub fn build_client() -> Client {
    Client::builder()
        .pool_max_idle_per_host(100)
        .build()
        .expect("Failed to build client")
}

pub fn build_request(client: &Client, config: &Config) -> reqwest::RequestBuilder {
    let method = match config.method.as_str() {
        "POST" => Method::POST,
        _ => Method::GET,
    };

    let mut req = client.request(method, &config.url);

    for (k, v) in &config.headers {
        req = req.header(k, v);
    }

    if let Some(body) = &config.body {
        req = req.body(body.clone());
    }

    req
}