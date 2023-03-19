use std::env;

use anyhow::{anyhow, Result};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {}

#[derive(Serialize)]
struct Response {
  msg: String,
}

async fn function_handler(_event: LambdaEvent<Request>) -> Result<Response, Error> {
  let weather_key = env_var("WEATHERAPICOM_API_KEY")?;
  let lat = env_var("LAT")?;
  let lon = env_var("LON")?;
  let github_token = env_var("GITHUB_TOKEN")?;

  let weather_text = gh_weather::update_status(&weather_key, &github_token, &lat, &lon)?;

  let resp = Response { msg: weather_text };

  Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .with_target(false)
    .without_time()
    .init();

  run(service_fn(function_handler)).await
}

fn env_var(name: &str) -> Result<String> {
  env::var(name)
    .map_err(|err| anyhow!("{err}: {name}"))
    .map(|s| s.trim_matches('"').to_owned())
}
