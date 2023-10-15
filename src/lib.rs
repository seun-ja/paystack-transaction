mod accept_payment;
mod cred;
mod error;

pub use cred::cred_from_env;

pub use error::{AuthError, ResponseError};
