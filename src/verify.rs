use std::time;

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::ResponseError;

// pub type VerificationResult =
pub type VerificationResult<T> = Result<T, ResponseError>;

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationData {
    pub status: bool,
    pub message: String,
    data: String,
}

/// Verify transactions
#[async_trait]
pub trait Verify {
    /// Verify the status of a transaction, given the reference of the transaction
    async fn verify_transaction(&self, reference: String) -> VerificationResult<VerificationData>;
}

/// Verify the status of a transaction, given the reference of the transaction
pub async fn verify_transaction(
    key: String,
    reference: String,
) -> Result<VerificationData, ResponseError> {
    let timeout = time::Duration::from_millis(10000);
    let http_client = Client::builder().timeout(timeout).build().unwrap();

    let url = format!("https://api.paystack.co/transaction/verify/{reference}");

    let response = http_client
        .get(url)
        .header("Authorization", format!("Bearer {}", key))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("Cache-Control", "no-cache")
        .send()
        .await
        .unwrap();

    let json_data: VerificationData = response.json().await.unwrap();

    Ok(json_data)
}
