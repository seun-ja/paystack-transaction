use std::time;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ResponseError;

// Ref: https://paystack.com/docs/payments/accept-payments/
#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentBuilder {
    // Required Data
    amount: f64,
    email: String,
    key: String,

    //Optional Data
    channel: Option<Channel>,
    currency: Option<Currency>,
    label: Option<String>,
    metadata: Option<String>,
    reference: Option<String>,
}

impl PaymentBuilder {
    pub fn init_payment(email: String, amount: f64, key: String) -> Self {
        Self {
            amount,
            email,
            key,
            channel: None,
            currency: None,
            label: None,
            metadata: None,
            reference: None,
        }
    }

    /// Build your `PaymentBuilder` object to be used to by `InitialisePay` to initiate Paystack payment
    pub fn build(self) -> InitialisePay {
        InitialisePay(self)
    }

    /// Amount in the subunit of the supported currency you are debiting customer. Do not pass this if creating subscriptions.
    pub fn amount(&self) -> f64 {
        self.amount
    }

    /// An array of payment channels to control what channels you want to make available to the user to make a payment with. Available channels include; ['card', 'bank', 'ussd', 'qr', 'mobile_money', 'bank_transfer']
    pub fn channel(&mut self, channel: Channel) {
        match channel {
            Channel::Card => self.channel = Some(Channel::Card),
            Channel::Bank => self.channel = Some(Channel::Bank),
            Channel::USSD => self.channel = Some(Channel::USSD),
            Channel::QR => self.channel = Some(Channel::QR),
            Channel::MobileMoney(info) => self.channel = Some(Channel::MobileMoney(info)),
            Channel::BankTransfer => self.channel = Some(Channel::BankTransfer),
        }
    }

    /// On of the supported currency [ `NGN`, `USD`, `GHS`, `ZAR`, `KES`]. The charge should be performed in. It defaults to your integration currency.
    pub fn currency(&mut self, currency: Currency) {
        self.currency = Some(currency)
    }

    /// Object containing any extra information you want recorded with the transaction. Fields within the custom_field object will show up on merchant receipt and within the transaction information on the Paystack Dashboard.
    pub fn metadata(&mut self, metadata: String) {
        self.metadata = Some(metadata)
    }

    /// String that replaces customer email as shown on the checkout form
    pub fn label(&mut self, label: String) {
        self.label = Some(label)
    }

    /// Unique case sensitive transaction reference. Only -,., =and alphanumeric characters allowed. If you do not pass this parameter, Paystack will generate a unique reference for you.
    pub fn reference(&mut self, reference: String) {
        self.reference = Some(reference)
    }

    // Convert this to trait, making it compatible with JavaScript functions
    /// Add runtime funtions such as `callback`, `onClose` and `onBankTransferConfirmationPending`
    pub fn add_fallback(&self, _f: fn() -> ()) {
        todo!()
    }

    fn json_builder(&self) -> serde_json::Value {
        let mut json = serde_json::to_value(self).unwrap();

        if let Value::Object(ref mut map) = json {
            let keys_to_remove: Vec<String> = map
                .iter()
                .filter(|&(_, v)| v.is_null())
                .map(|(k, _)| k.clone())
                .collect();

            for k in keys_to_remove {
                map.remove(&k);
            }
        }

        json
    }
}

pub struct InitialisePay(PaymentBuilder);

impl InitialisePay {
    /// Send Transaction
    pub async fn send(&self) -> Result<(), ResponseError> {
        let timeout = time::Duration::from_millis(10000);
        let http_client = Client::builder().timeout(timeout).build().unwrap();

        let data = self.0.json_builder();

        http_client
            .post("https://api.paystack.co/transaction/initialize")
            .header("Authorization", format!("Bearer {}", self.0.key))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Cache-Control", "no-cache")
            .json(&data)
            .send()
            .await
            .map_err(|e| ResponseError::PayStackError(e.to_string()))
            .unwrap();

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Currency {
    NGN,
    USD,
    GHS,
    ZAR,
    KES,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Channel {
    Card,
    Bank,
    USSD,
    QR,
    MobileMoney(MobileMoneyInfo),
    BankTransfer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MobileMoneyInfo {
    phone: u32,
    provider: String,
}

mod test {
    use serde_json::Value;

    use super::{Channel, PaymentBuilder};

    fn init_Builder() -> PaymentBuilder {
        let mut builder = PaymentBuilder::init_payment(
            "test@example.com".to_string(),
            100.0,
            "secret_key".to_string(),
        );

        builder.channel(Channel::Card);
        builder.label("label".to_string());
        builder.reference("reference".to_string());

        builder
    }

    #[test]
    fn json_response() {
        let builder = init_Builder();
        let json_builder = builder.json_builder();

        let data = r#"{
            "amount":100.0,
            "channel":"Card",
            "email":"test@example.com",
            "key":"secret_key",
            "label":"label",
            "reference":"reference"
        }"#;

        let json: Value = serde_json::from_str(data).unwrap();

        assert_eq!(json, json_builder)
    }
}
