//! # Paystack Transaction
//!
//! A Simple to use package to use Paystack with Rust
//!
//! # Usage
//! ```no_run
//! use paystack_transaction::{cred_from_env, PaymentBuilder, Payment, MobileMoney};
//!
//! async fn build() {
//!     let key = cred_from_env("SECRET_KEY".to_string()).unwrap();
//!
//!     let mut builder = Payment::builder(
//!         "test@example.com".to_string(),
//!         100.0,
//!         key,
//!     );
//!
//!     builder.mobile_money(
//!         MobileMoney {
//!            phone: "08123456789".to_string(),
//!            provider: "MTN".to_string(),
//!         }
//!     );
//!     builder.label("label".to_string());
//!     builder.reference("reference".to_string());
//!
//!     builder.build().send().await.unwrap();
//! }
//! ```

mod channels;
mod cred;
mod error;
mod initialize;
mod verify;

pub use initialize::{Currency, Payment, PaymentBuilder};

pub use cred::{cred_from_env, expose_secret};

pub use channels::MobileMoney;

pub use error::{AuthError, ResponseError};

pub use verify::{verify_transaction, VerificationData, VerificationResult, Verify};
