# Key Pair Based Authentication

```rust
use binance_sdk::sub_account;
use binance_sdk::config;

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .private_key(config::PrivateKey::File("your-private-key-file-path".to_string())) // Provide the private key file path
    .private_key_passphrase("your-passphrase".to_string()) // Optional: Required if the private key is encrypted
    .build()?;

let client = sub_account::SubAccountRestApi::production(configuration);
let params = sub_account::rest_api::GetSummaryOfSubAccountsMarginAccountParams::default();
let response = client.get_summary_of_sub_accounts_margin_account(params).await?;
```
