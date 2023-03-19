use std::env;

use anyhow::{anyhow, Result};
use dotenvy::dotenv;

fn main() -> Result<()> {
  dotenv()?;

  let weather_key = env_var("WEATHERAPICOM_API_KEY")?;
  let lat = env_var("LAT")?;
  let lon = env_var("LON")?;
  let github_token = env_var("GITHUB_TOKEN")?;

  gh_weather::update_status(&weather_key, &github_token, &lat, &lon)
}

fn env_var(name: &str) -> Result<String> {
  env::var(name).map_err(|err| anyhow!("{err}: {name}"))
}
