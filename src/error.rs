//ref: https://paystack.com/docs/api/errors

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Your public key must be set")]
    NoPublicKey,
}

#[derive(thiserror::Error, Debug)]
pub enum ResponseError {
    #[error("Paystack Error: {0}")]
    PayStackError(String),
}