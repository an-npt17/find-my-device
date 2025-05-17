use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub ip: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub fn get_location() -> Result<Location, Box<dyn Error>> {
    let domain_url = "https://api.ipify.org";
    let params = [("format", "json")];
    let url = reqwest::Url::parse_with_params(domain_url, &params)?;
    let response = reqwest::blocking::get(url)?;

    if !response.status().is_success() {
        return Err(format!("API request failed with status: {}", response.status()).into());
    }

    let api_response: serde_json::Value = response.json()?;

    let location = Location {
        ip: api_response["ip"].as_str().unwrap_or("unknown").to_string(),
        timestamp: chrono::Utc::now(),
    };

    Ok(location)
}
