use std::{
    str::FromStr,
    time::{Duration, SystemTime},
};

use allfeat_faucet_shared::TransferRequest;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use subxt::utils::AccountId32;
use tokio::spawn;
use tracing::info;

use crate::{captcha::verify_captcha, chain::transfer_to, FaucetState, LastClaims};

pub async fn handle_transfer(
    State(state): State<FaucetState>,
    Json(req): Json<TransferRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    if !can_claim(&req.address, &state.last_claims).await {
        return Err(StatusCode::UNAUTHORIZED);
    }

    verify_captcha(&req.token)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let account_id = AccountId32::from_str(&req.address).map_err(|_| StatusCode::BAD_REQUEST)?;
    spawn(async move {
        if let Err(e) = transfer_to(account_id, state, req.client_id).await {
            tracing::error!("Transfer failed: {}", e);
        }
    });
    Ok(StatusCode::ACCEPTED)
}

/// Function guard to ensure the cooldown claim
async fn can_claim(address: &str, last_claims: &LastClaims) -> bool {
    match last_claims.read().await.get(address) {
        Some(&last_time) => match SystemTime::now().duration_since(last_time) {
            Ok(elapsed) => elapsed >= Duration::from_secs(24 * 3600),
            Err(_) => false,
        },
        None => true,
    }
}
