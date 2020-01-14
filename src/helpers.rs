use reqwest::Error;
use serde_json::{Value};

#[tokio::main]
pub async fn fetch_json(url: &str) -> Result<Value, Error> {
    let response = reqwest::get(url)
        .await?
        .text()
        .await?;

    let data: Value = serde_json::from_str(&response)
        .expect("error parsing json");

    Ok(data)
}