use std::env;

use anyhow::{anyhow, Result};
#[cfg(not(feature = "prod"))]
use dotenvy::dotenv;

#[cfg(not(feature = "prod"))]
fn main() -> Result<()> {
  dotenv()?;
  run()
}

#[cfg(feature = "prod")]
fn main() -> Result<()> {
  run()
}

fn run() -> Result<()> {
  let weather_key = env_var("WEATHERAPICOM_API_KEY")?;
  let lat = env_var("LAT")?;
  let lon = env_var("LON")?;
  let github_token = env_var("GITHUB_TOKEN")?;

  gh_weather::update_status(&weather_key, &github_token, &lat, &lon).map(|_| ())
}

fn env_var(name: &str) -> Result<String> {
  env::var(name).map_err(|err| anyhow!("{err}: {name}"))
}
