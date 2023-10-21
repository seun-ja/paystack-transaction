use serde::{Deserialize, Serialize};

// Selection of Banks available e.g "Guaranty Trust Bank"
#[derive(Debug, Serialize, Deserialize)]
pub enum Bank {}

/// Bank Transfer data
#[derive(Debug, Serialize, Deserialize)]
#[serde[rename_all = "snake_case"]]
pub struct BankTransfer {}

/// Card transaction
#[derive(Debug, Serialize, Deserialize)]
#[serde[rename_all = "snake_case"]]
pub struct Card {}

/// Eft transaction
#[derive(Debug, Serialize, Deserialize)]
#[serde[rename_all = "snake_case"]]
pub struct Eft {}

/// Mobile money object data
#[derive(Debug, Serialize, Deserialize)]
#[serde[rename_all = "snake_case"]]
pub struct MobileMoney {
    /// Receiver phone number
    pub phone: String,
    /// Network provider
    pub provider: String,
}

/// USSD transaction
#[derive(Debug, Serialize, Deserialize)]
#[serde[rename_all = "snake_case"]]
pub struct Ussd {}

/// QR transaction
#[derive(Debug, Serialize, Deserialize)]
#[serde[rename_all = "snake_case"]]
pub struct QR {}
