use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Deserialize, Debug)]
pub struct TemperatureInput {
    pub celsius: f64,
}

#[derive(Serialize, Debug)]
pub struct TemperatureOutput {
    pub fahrenheit: f64,
}

pub fn convert_temp(input: &TemperatureInput) -> Result<TemperatureOutput> {
    let fahrenheit = input.celsius * 1.8 + 32.0;
    Ok(TemperatureOutput { fahrenheit })
}