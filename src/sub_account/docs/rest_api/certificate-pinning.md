# Certificate Pinning

```rust
use std;
use reqwest;

use binance_sdk::sub_account;
use binance_sdk::config;

let pem = std::fs::read("/path/to/pinned_cert.pem");
let cert = reqwest::Certificate::from_pem(&pem);

let custom_agent = config::HttpAgent(std::sync::Arc::new(|builder: reqwest::ClientBuilder| {
    builder.add_root_certificate(cert.clone())
}));

let configuration = config::ConfigurationRestApi::builder()
    .api_key("your-api-key")
    .api_secret("your-api-secret")
    .agent(custom_agent)
    .build()?;

let client = sub_account::SubAccountRestApi::production(configuration);
let params = sub_account::rest_api::GetSummaryOfSubAccountsMarginAccountParams::default();
let response = client.get_summary_of_sub_accounts_margin_account(params).await?;
```
