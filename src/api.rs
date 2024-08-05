use reqwest;
use serde::Deserialize;
use anyhow::{Context, Result};

#[derive(Deserialize, Debug)]
pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
    // pub lat: f64,
    // pub lon: f64,
    // pub tz_id: String,
    // pub localtime_epoch: i64,
    pub localtime: String,
}

#[derive(Deserialize, Debug)]
pub struct Condition {
    pub text: String,
    pub icon: String,
    pub code: i32,
}

#[derive(Deserialize, Debug)]
pub struct Current {
    // pub last_updated_epoch: i64,
    pub last_updated: String,
    pub temp_c: f64,
    pub temp_f: f64,
    pub is_day: i32,
    pub condition: Condition,
    pub wind_mph: f64,
    pub wind_kph: f64,
    pub wind_degree: i32,
    pub wind_dir: String,
    // pub pressure_mb: f64,
    // pub pressure_in: f64,
    // pub precip_mm: f64,
    // pub precip_in: f64,
    // pub humidity: i32,
    // pub cloud: i32,
    // pub feelslike_c: f64,
    // pub feelslike_f: f64,
    // pub vis_km: f64,
    // pub vis_miles: f64,
    pub uv: f64,
    pub gust_mph: f64,
    pub gust_kph: f64,
}

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub location: Location,
    pub current: Current,
}

// API key constant
const API_KEY: &str = "f8729ee609114e91aca133756240508";

// Function to fetch weather data
pub async fn fetch_weather(city: &str) -> Result<ApiResponse> {
    let url = format!("https://api.weatherapi.com/v1/current.json?key={}&q={}", API_KEY, city);
    let client = reqwest::Client::new();

    // Send request and handle potential errors
    let response = client.get(&url)
        .send()
        .await
        .with_context(|| format!("Failed to send request to {}", url))?;

    // Check for HTTP errors
    if response.status().is_success() {
        // Deserialize the response body
        let weather: ApiResponse = response.json()
            .await
            .with_context(|| "Failed to deserialize response body")?;
        Ok(weather)
    } else {
        // Handle different HTTP error responses
        match response.status() {
            reqwest::StatusCode::FORBIDDEN => {
                Err(anyhow::anyhow!("403 Forbidden: The API key is invalid or you do not have permission to access the requested resource"))
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err(anyhow::anyhow!("404 Not Found: The requested city '{}' was not found", city))
            }
            _ => {
                Err(anyhow::anyhow!("Received unexpected status code {}: {}", response.status(), response.text().await.unwrap_or_default()))
            }
        }
    }
}