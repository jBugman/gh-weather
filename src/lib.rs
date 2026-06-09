use anyhow::{anyhow, Context, Result};

mod weather;

use crate::weather::fetch_weather;

pub fn update_status(
  weather_api_key: &str,
  guthub_access_token: &str,
  lat: &str,
  lon: &str,
) -> Result<String> {
  let weather = fetch_weather(weather_api_key, lat, lon).context("failed to fetch weather")?;

  let condition = weather.condition;

  let emoji = emoji_from_code(condition.code);
  let text = condition.text;
  println!("Setting status to {emoji} ({text})");

  update_github_status_emoji(guthub_access_token, emoji).context("failed to set status")?;

  Ok(text)
}

// Too bad there is not enough different emojis.
fn emoji_from_code(code: u32) -> &'static str {
  match code {
    1000 => "☀️",  // Sunny
    1003 => "🌤️", // Partly cloudy
    1006 => "⛅️",  // Cloudy
    1009 => "☁️",  // Overcast
    1030 => "🌫️", // Mist
    1063 => "🌦️", // Patchy rain possible
    1066 => "☁️",  // Patchy snow possible
    1069 => "☁️",  // Patchy sleet possible
    1072 => "☁️",  // Patchy freezing drizzle possible
    1087 => "🌩️", // Thundery outbreaks possible
    1114 => "❄️",  // Blowing snow
    1117 => "❄️",  // Blizzard
    1135 => "🌫️", // Fog
    1147 => "🌫️", // Freezing fog
    1150 => "🌦️", // Patchy light drizzle
    1153 => "🌦️", // Light drizzle
    1168 => "🌧️", // Freezing drizzle
    1171 => "🌧️", // Heavy freezing drizzle
    1180 => "🌦️", // Patchy light rain
    1183 => "🌧️", // Light rain
    1186 => "🌧️", // Moderate rain at times
    1189 => "🌧️", // Moderate rain
    1192 => "🌧️", // Heavy rain at times
    1195 => "🌧️", // Heavy rain
    1198 => "🌧️", // Light freezing rain
    1201 => "🌧️", // Moderate or heavy freezing rain
    1204 => "🌦️", // Light sleet
    1207 => "🌧️", // Moderate or heavy sleet
    1210 => "🌨️", // Patchy light snow
    1213 => "🌨️", // Light snow
    1216 => "🌨️", // Patchy moderate snow
    1219 => "🌨️", // Moderate snow
    1222 => "❄️",  // Patchy heavy snow
    1225 => "❄️",  // Heavy snow
    1237 => "❄️",  // Ice pellets
    1240 => "🌧️", // Light rain shower
    1243 => "🌧️", // Moderate or heavy rain shower
    1246 => "🌧️", // Torrential rain shower
    1249 => "🌦️", // Light sleet showers
    1252 => "🌧️", // Moderate or heavy sleet showers
    1255 => "🌨️", // Light snow showers
    1258 => "🌨️", // Moderate or heavy snow showers
    1261 => "🌨️", // Light showers of ice pellets
    1264 => "🌨️", // Moderate or heavy showers of ice pellets
    1273 => "⛈️",  // Patchy light rain with thunder
    1276 => "⛈️",  // Moderate or heavy rain with thunder
    1279 => "🌩️", // Patchy light snow with thunder
    1282 => "🌩️", // Moderate or heavy snow with thunder
    _ => "🤔",
  }
}

pub fn update_github_status_emoji(access_token: &str, emoji: &str) -> Result<()> {
  const URL: &str = "https://api.github.com/graphql";

  let payload = format!(
    "{{ \
      \"query\": \"mutation {{ \
        changeUserStatus(input: {{ emoji: \\\"{emoji}\\\" }}) {{ \
          status {{ \
            emoji \
          }} \
        }} \
      }} \
    \"}}"
  );

  let resp = ureq::post(URL)
    .header("Authorization", &format!("Bearer {access_token}"))
    .send(payload.as_str())?
    .body_mut()
    .read_json::<serde_json::Value>()
    .context("failed to call github graphql api")?;

  let resp_emoji = resp["data"]["changeUserStatus"]["status"]
    .get("emoji")
    .and_then(serde_json::Value::as_str)
    .ok_or(anyhow!("failed to parse graphql response"))?;

  if resp_emoji != emoji {
    return Err(anyhow!("resulting status mismatch"));
  }
  Ok(())
}
