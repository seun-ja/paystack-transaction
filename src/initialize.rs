use std::time;

use async_trait::async_trait;
use reqwest::Client;
use secrecy::Secret;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    channels::{Bank, BankTransfer, Card, MobileMoney, QR, Ussd},
    expose_secret,
    verify::{VerificationData, Verify},
    ResponseError,
};

/// Building blocks for initiating a Paystack Payment
#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentBuilder {
    // Required Data
    amount: f64,
    email: String,
    key: String,

    //Channel Options
    #[serde(skip_serializing_if = "Option::is_none")]
    bank: Option<Bank>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bank_transfer: Option<BankTransfer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card: Option<Card>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mobile_money: Option<MobileMoney>,
    #[serde(skip_serializing_if = "Option::is_none")]
    qr: Option<QR>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ussd: Option<Ussd>,

    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reference: Option<String>,
}

impl PaymentBuilder {
    /// Initiate your `PaymentBuilder` taking in the basic requirements as args.
    ///
    /// key can be derived using the `cred_from_env`
    pub fn init_payment(email: String, amount: f64, key: Secret<String>) -> Self {
        Self {
            amount,
            email,
            key: expose_secret(key),
            currency: None,
            label: None,
            metadata: None,
            reference: None,
            bank: None,
            bank_transfer: None,
            card: None,
            mobile_money: None,
            qr: None,
            ussd: None,
        }
    }

    /// create your `Payment` to initiate Paystack payment
    pub fn build(self) -> Payment {
        Payment(self)
    }

    /// Amount in the subunit of the supported currency you are debiting customer. Do not pass this if creating subscriptions.
    pub fn amount(&self) -> f64 {
        self.amount
    }

    /// On of the supported currency [ `NGN`, `USD`, `GHS`, `ZAR`, `KES`]. The charge should be performed in. It defaults to your integration currency.
    pub fn currency(&mut self, currency: Currency) {
        match currency {
            Currency::GHS => self.currency = Some("GHS".to_string()),
            Currency::NGN => self.currency = Some("NGN".to_string()),
            Currency::USD => self.currency = Some("USD".to_string()),
            Currency::ZAR => self.currency = Some("ZAR".to_string()),
            Currency::KES => self.currency = Some("KES".to_string()),
        }
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

    /// Set your mobile money data
    pub fn mobile_money(&mut self, mobile_money: MobileMoney) {
        self.mobile_money = Some(mobile_money)
    }

    /// Set your card data
    pub fn card(&mut self, card: Card) {
        self.card = Some(card)
    }

    /// Set your bank data
    pub fn bank(&mut self, bank: Bank)  {
        self.bank = Some(bank)
    }

    /// Set your bank transfer data
    pub fn bank_transfer(&mut self, bank_transfer: BankTransfer) {
        self.bank_transfer = Some(bank_transfer)
    }

    /// Set your ussd data
    pub fn ussd(&mut self, ussd: Ussd) {
        self.ussd = Some(ussd)
    }

    /// Set your qr data
    pub fn qr(&mut self, qr: QR) {
        self.qr = Some(qr)
    }

    // Convert this to trait, making it compatible with JavaScript functions
    /// Add runtime funtions such as `callback`, `onClose` and `onBankTransferConfirmationPending`
    pub fn add_fallback(&self, f: fn() -> ()) {
        f()
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

/// Data wrapper for payment ready to send for initialization
pub struct Payment(PaymentBuilder);

impl Payment {
    /// Build your `PaymentBuilder` object to be used to by `Payment` to initiate Paystack payment
    pub fn builder(email: String, amount: f64, key: Secret<String>) -> PaymentBuilder {
        PaymentBuilder {
            amount,
            email,
            key: expose_secret(key),
            currency: None,
            label: None,
            metadata: None,
            reference: None,
            bank: None,
            bank_transfer: None,
            card: None,
            mobile_money: None,
            qr: None,
            ussd: None,
        }
    }

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

#[async_trait]
impl Verify for Payment {
    async fn verify_transaction(
        &self,
        reference: String,
    ) -> Result<VerificationData, ResponseError> {
        let timeout = time::Duration::from_millis(10000);
        let http_client = Client::builder().timeout(timeout).build().unwrap();

        let url = format!("https://api.paystack.co/transaction/verify/{reference}");

        let response = http_client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.0.key))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Cache-Control", "no-cache")
            .send()
            .await
            .unwrap();

        let json_data: VerificationData = response.json().await.unwrap();

        Ok(json_data)
    }
}

/// Supported Currencies
#[derive(Debug, Serialize, Deserialize)]
pub enum Currency {
    /// Nigerian Naira
    NGN,
    /// US Dollars
    USD,
    /// Ghanaian Cedis
    GHS,
    /// South African Rand
    ZAR,
    /// Kenyan Shillings
    KES,
}

/// Available payment channels
#[derive(Debug, Serialize, Deserialize)]
pub enum Channel {
    Card,
    Bank,
    Ussd,
    QR,
    MobileMoney,
    BankTransfer,
}

// impl Channel {
//     /// An array of payment channels to control what channels you want to make available to the user to make a payment with. Available channels include; ['card', 'bank', 'ussd', 'qr', 'mobile_money', 'bank_transfer']
//     pub fn channel(&self) -> Channel {
//         match &self {
//             Channel::Card => todo!(),
//             Channel::Bank => todo!(),
//             Channel::USSD => todo!(),
//             Channel::QR => todo!(),
//             Channel::MobileMoney => todo!(),
//             Channel::BankTransfer => todo!(),
//         }
//     }

//     pub fn mobile_money(&mut self, phone: String, provider: String) -> MobileMoney {
//         todo!()
//     }

//     pub fn card(&mut self, card: Card) {}

//     pub fn bank(&mut self, bank: Bank) {}

//     pub fn bank_transfer(&mut self, bank_transfer: BankTransfer) {}

//     pub fn ussd(&mut self, ussd: USSD) {}

//     pub fn qr(&mut self, qr: QR) {}
// }

mod test {
    #[test]
    fn json_response() {
        let mut builder = crate::Payment::builder(
            "test@example.com".to_string(),
            100.0,
            "secret_key".to_string().into(),
        );

        builder.mobile_money(crate::channels::MobileMoney {
            phone: "08123456789".to_string(),
            provider: "MTN".to_string(),
        });

        builder.label("label".to_string());
        builder.reference("reference".to_string());
        builder.currency(crate::Currency::NGN);
        
        let json_builder = builder.json_builder();

        let data = r#"{
            "amount":100.0,
            "email":"test@example.com",
            "mobile_money": {
                "phone": "08123456789",
                "provider": "MTN"
            },
            "currency": "NGN",
            "key":"secret_key",
            "label":"label",
            "reference":"reference"
        }"#;

        let json: serde_json::Value = serde_json::from_str(data).unwrap();

        assert_eq!(json, json_builder)
    }
}
