mod api;
mod cli;
mod tempconv;

use clap::Parser;
use anyhow::Result;
use api::ApiResponse;
use tempconv::{TemperatureInput, TemperatureOutput, convert_temp};

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = cli::Args::parse();

    // Fetch weather data
    match api::fetch_weather(&args.city).await {
        Ok(weather) => {
            println!("Weather in {}: {:?}", args.city, weather);

            // Convert the temperature from Celsius to Fahrenheit
            let temp_input = TemperatureInput { celsius: weather.current.temp_c };
            let temp_output = convert_temp(&temp_input)?;
            println!(
                "Temperature in {}: {}°C / {}°F", 
                args.city, 
                weather.current.temp_c, 
                temp_output.fahrenheit
            );
        }
        Err(e) => {
            eprintln!("Error fetching weather data: {}", e);
        }
    }

    Ok(())
}