use std::env;

use axum::response::IntoResponse;
use reqwest::Client;
use serde::Deserialize;
use tracing::{debug, info, instrument};

#[derive(Deserialize)]
pub struct TurnstileResponse {
    success: bool,
    #[serde(default)]
    #[allow(dead_code)]
    error_codes: Vec<String>,
}

fn get_cf_secret_key() -> String {
    env::var("CF_SECRET").unwrap_or_else(|_| "1x0000000000000000000000000000000AA".to_string())
}

pub async fn cf_sitekey() -> impl IntoResponse {
    info!("Received CF_SITEKEY request.");
    env::var("CF_SITEKEY").unwrap_or_else(|_| "1x00000000000000000000AA".to_string())
}

#[instrument("verify_captcha", skip_all)]
pub async fn verify_captcha(cf_token: &String) -> Result<(), String> {
    debug!("Verifying cloudflare challenge token: {}", cf_token);
    let secret_key = get_cf_secret_key();

    let client = Client::new();
    let res = client
        .post("https://challenges.cloudflare.com/turnstile/v0/siteverify")
        .form(&[("secret", &secret_key), ("response", cf_token)])
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {e}"))?;

    let body: TurnstileResponse = res
        .json()
        .await
        .map_err(|e| format!("Invalid response: {e}"))?;

    if body.success {
        Ok(())
    } else {
        Err("Captcha verification failed".into())
    }
}
