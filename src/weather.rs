use anyhow::Result;
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Response {
  current: Weather,
}

#[derive(Deserialize)]
pub struct Weather {
  pub condition: Condition,
}

#[derive(Deserialize)]
pub struct Condition {
  pub text: String,
  pub code: u32,
}

pub fn fetch_weather(api_key: &str, lat: &str, lon: &str) -> Result<Weather> {
  let url =
    format!("https://api.weatherapi.com/v1/current.json?key={api_key}&q={lat},{lon}&aqi=no)");
  let resp = ureq::get(&url).call()?.into_json::<Response>()?;
  Ok(resp.current)
}
