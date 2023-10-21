use std::env;

use dotenv::dotenv;
use secrecy::{ExposeSecret, Secret};

use crate::AuthError;

/// Helper function that looks for Secret Key from your `.env` file and returns a `Secret<String>`.
///
/// # Example
/// ```no_run
/// use paystack_transaction::{cred_from_env, PaymentBuilder, Payment};
///
/// async fn build() {
///     let key = cred_from_env("SECRET_KEY".to_string()).unwrap();
///
///     let builder = PaymentBuilder::init_payment(
///         "test@example.com".to_string(),
///         100.0,
///         key,
///     );
///
///     builder.build().send().await.unwrap();
/// }
/// ```
pub fn cred_from_env(env_key: String) -> Result<Secret<String>, AuthError> {
    dotenv().ok();

    let pk = env::var(env_key)
        .map_err(|_| AuthError::NoPublicKey)
        .unwrap();

    Ok(Secret::new(pk))
}

/// Helper for unmasking `Secret` wrapper
pub fn expose_secret(secret: Secret<String>) -> String {
    secret.expose_secret().to_owned()
}
