use std::time;

use reqwest::Client;
use secrecy::{Secret, ExposeSecret};
use serde_json::json;

use crate::{cred_from_env, ResponseError};

// Ref: https://paystack.com/docs/payments/accept-payments/
struct PaymentBuilder {
    // Reuquired Data
    amount: f64,
    email: String,
    key: Secret<String>,

    //Optional Data
    channel: Option<Channel>,
    currency: Option<Currency>,
    label: Option<String>,
    reference: Option<String>,
    //TODO: Consider converting these to enum then execute respective functions
    callback: Option<Box<dyn Fn()>>,
    on_bank_transfer_confirmation_pending: Option<Box<dyn Fn()>>,
    on_close: Option<Box<dyn Fn()>>, // TODO: Make it compatible with JavaScript function
}

impl PaymentBuilder {
    pub fn init_payment(email: String, amount: f64, key: Secret<String>) -> Self {
        Self {
            amount,
            email,
            key,
            channel: None,
            currency: None,
            label: None,
            reference: None,
            callback: None,
            on_bank_transfer_confirmation_pending: None,
            on_close: None,
        }
    }

    pub fn build(self) -> Pay {
        Pay(self)
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn channel(&mut self, channel: Channel) {
        match channel {
            Channel::Card => todo!(),
            Channel::Bank => todo!(),
            Channel::USSD => todo!(),
            Channel::QR => todo!(),
            Channel::MobileMoney(info) => {
                self.channel = Some(Channel::MobileMoney(info))
            },
            Channel::BankTransfer => todo!(),
        }
    }

    pub fn currency(&mut self, currency: Currency) {
        self.currency = Some(currency)
    }

    pub fn label(&mut self, label: String) {
        self.label = Some(label)
    }

    pub fn reference(&mut self, reference: String) {
        self.reference = Some(reference)
    }

    fn json_builder(&self) -> serde_json::Value {
        json!({
            "amount": self.amount,
            "email": self.email,
        })
    }
}

struct Pay (PaymentBuilder);

impl Pay {
    pub async fn send(&self) -> Result<(), ResponseError> {
        let timeout = time::Duration::from_millis(10000);
        let http_client = Client::builder().timeout(timeout).build().unwrap();

        let data = self.0.json_builder();

        http_client
            .post("https://api.paystack.co/transaction/initialize")
            .header(
                "Authorization",
                format!("Bearer {}", self.0.key.expose_secret()),
            )
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Cache-Control", "no-cache")
            .json(&data)
            .send()
            .await
            .map_err(|e| {
                ResponseError::PayStackError(e.to_string())
            })
            .unwrap();

        Ok(())
    }
}

#[derive(Debug)]
enum Currency {
    NGN,
    USD,
    GHS,
    ZAR,
    KES,
}

#[derive(Debug)]
enum Channel {
    Card,
    Bank,
    USSD,
    QR,
    MobileMoney(MobileMoneyInfo),
    BankTransfer,
}

#[derive(Debug)]
struct MobileMoneyInfo {
    phone: u32,
    provider: String,
}

mod test {
    use super::PaymentBuilder;
}
