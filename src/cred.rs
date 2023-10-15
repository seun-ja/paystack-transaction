use std::env;

use dotenv::dotenv;
use secrecy::Secret;

use crate::AuthError;

pub fn cred_from_env(env_key: String) -> Result<Secret<String>, AuthError> {
    dotenv().ok();

    let pk = env::var(env_key)
        .map_err(|_| AuthError::NoPublicKey)
        .unwrap();

    Ok(Secret::new(pk))
}