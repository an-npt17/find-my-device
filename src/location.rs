use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub ip: String,
    pub latitude: f64,
    pub longitude: f64,
    pub city: String,
    pub region: String,
    pub country: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub fn get_location() -> Result<Location, Box<dyn Error>> {
    // This uses a free IP geolocation API
    // In a production app, you might want to use a more reliable service with an API key
    let response = reqwest::blocking::get("https://ipapi.co/json/")?;

    if !response.status().is_success() {
        return Err(format!("API request failed with status: {}", response.status()).into());
    }

    let api_response: serde_json::Value = response.json()?;

    let location = Location {
        ip: api_response["ip"].as_str().unwrap_or("unknown").to_string(),
        latitude: api_response["latitude"].as_f64().unwrap_or(0.0),
        longitude: api_response["longitude"].as_f64().unwrap_or(0.0),
        city: api_response["city"]
            .as_str()
            .unwrap_or("unknown")
            .to_string(),
        region: api_response["region"]
            .as_str()
            .unwrap_or("unknown")
            .to_string(),
        country: api_response["country_name"]
            .as_str()
            .unwrap_or("unknown")
            .to_string(),
        timestamp: chrono::Utc::now(),
    };

    Ok(location)
}
