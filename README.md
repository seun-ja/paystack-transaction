# Paystack Transaction

A Simple package to use Paystack with Rust

## Usage

```rust
#[tokio::main]
async fn main() {
    let key = cred_from_env("SECRET_KEY".to_string()).unwrap();

    let mut builder = Payment::builder(
        "test@example.com".to_string(),
        100.0,
        key,
    );

    builder.channel(Channel::Card);
    builder.label("label".to_string());
    builder.reference("reference".to_string());

    builder.build().send().await.unwrap();
}
```

## Installation

```toml
[dependencies]
paystack-transaction = "0.1.2"
```
