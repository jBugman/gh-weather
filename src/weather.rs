use anyhow::Result;
use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer};
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Response {
  current: Weather,
}

#[derive(Deserialize)]
pub struct Weather {
  #[serde(deserialize_with = "datetime_deserializer")]
  pub last_updated: NaiveDateTime,
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

fn datetime_deserializer<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
  D: Deserializer<'de>,
{
  let s: String = Deserialize::deserialize(deserializer)?;
  // 2023-03-08 17:30
  NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M").map_err(serde::de::Error::custom)
}
