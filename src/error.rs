//ref: https://paystack.com/docs/api/errors

/// Authentication Error types
#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Your public key must be set")]
    NoPublicKey,
}

/// Response Error wrapping error response from Paystack API
#[derive(thiserror::Error, Debug)]
pub enum ResponseError {
    #[error("Paystack Error: {0}")]
    PayStackError(String),
    #[error("{0}; Couldn't verify transaction")]
    TransactionVerificationError(bool),
}
