use chrono::{Local, Timelike};
use serde_json::Value;
use std::collections::HashMap;
use std::env;

const WEATHER_CODES: &[(&str, &str)] = &[
    ("113", "☀️"),
    ("116", "⛅️"),
    ("119", "☁️"),
    ("122", "☁️"),
    ("143", "🌧"),
    ("176", "🌧"),
    ("179", "🌧"),
    ("182", "🌧"),
    ("185", "🌧"),
    ("200", "⛈"),
    ("227", "🌨"),
    ("230", "❄️"),
    ("248", "🌫"),
    ("260", "🌫"),
    ("263", "🌧"),
    ("266", "🌧"),
    ("281", "🌧"),
    ("284", "🌧"),
    ("293", "🌧"),
    ("296", "🌧"),
    ("299", "🌧"),
    ("302", "🌧"),
    ("305", "🌧"),
    ("308", "🌧"),
    ("311", "🌧"),
    ("314", "🌧"),
    ("317", "🌧"),
    ("320", "🌧"),
    ("323", "🌧"),
    ("326", "🌧"),
    ("329", "❄️"),
    ("332", "❄️"),
    ("335", "❄️"),
    ("338", "❄️"),
    ("350", "🌧"),
    ("353", "🌧"),
    ("356", "🌧"),
    ("359", "🌧"),
    ("362", "🌧"),
    ("365", "🌧"),
    ("368", "🌧"),
    ("371", "❄️"),
    ("374", "🌧"),
    ("377", "🌧"),
    ("386", "⛈"),
    ("389", "🌩"),
    ("392", "⛈"),
    ("395", "❄️"),
];

fn get_weather_icon(code: &str) -> &'static str {
    WEATHER_CODES
        .iter()
        .find(|(c, _)| *c == code)
        .map(|(_, icon)| *icon)
        .unwrap_or("❓")
}

fn format_time(time: &str) -> String {
    let formatted = time.replace("00", "");
    if formatted.is_empty() {
        "0".to_string()
    } else {
        formatted
    }
}

fn format_temp(temp: &str) -> String {
    format!("{}°", temp).chars().take(3).collect::<String>()
}

fn format_chances(hour: &Value) -> String {
    let chances = vec![
        ("chanceoffog", "Fog"),
        ("chanceoffrost", "Frost"),
        ("chanceofovercast", "Overcast"),
        ("chanceofrain", "Rain"),
        ("chanceofsnow", "Snow"),
        ("chanceofsunshine", "Sunshine"),
        ("chanceofthunder", "Thunder"),
        ("chanceofwindy", "Wind"),
    ];

    let mut conditions = Vec::new();
    for (event, label) in chances {
        if let Some(value) = hour[event].as_str() {
            if let Ok(percent) = value.parse::<i32>() {
                if percent > 0 {
                    conditions.push(format!("{} {}%", label, value));
                }
            }
        }
    }
    conditions.join(", ")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get location from command line arguments, return error if not provided
    let location = env::args()
        .nth(1)
        .ok_or("Error: City location is required. Usage: ./weather-rs \"City, Country\"")?;

    // Fetch weather data
    let client = reqwest::blocking::Client::new();
    let url = format!("https://wttr.in/{}?format=j1", location);
    let response = client.get(&url).send()?;

    let weather: Value = response.json()?;

    let current = &weather["current_condition"][0];
    let weather_code = current["weatherCode"].as_str().unwrap_or("113");
    let feels_like = current["FeelsLikeC"].as_str().unwrap_or("0");

    let mut data = HashMap::new();

    // Set main text
    data.insert(
        "text",
        format!("{} {}°", get_weather_icon(weather_code), feels_like),
    );

    // Build tooltip
    let mut tooltip = String::new();
    let weather_desc = current["weatherDesc"][0]["value"]
        .as_str()
        .unwrap_or("Unknown");
    let temp_c = current["temp_C"].as_str().unwrap_or("0");
    let wind_speed = current["windspeedKmph"].as_str().unwrap_or("0");
    let humidity = current["humidity"].as_str().unwrap_or("0");

    tooltip.push_str(&format!("<b>{} {}</b>\n", weather_desc, temp_c));
    tooltip.push_str(&format!("Feels like: {}°\n", feels_like));
    tooltip.push_str(&format!("Wind: {}Km/h\n", wind_speed));
    tooltip.push_str(&format!("Humidity: {}%\n", humidity));

    let now = Local::now();
    let current_hour = now.hour();

    // Add daily forecasts
    if let Some(weather_array) = weather["weather"].as_array() {
        for (i, day) in weather_array.iter().enumerate() {
            tooltip.push_str("\n<b>");
            match i {
                0 => tooltip.push_str("Today, "),
                1 => tooltip.push_str("Tomorrow, "),
                _ => {}
            }

            let date = day["date"].as_str().unwrap_or("");
            tooltip.push_str(&format!("{}</b>\n", date));

            let max_temp = day["maxtempC"].as_str().unwrap_or("0");
            let min_temp = day["mintempC"].as_str().unwrap_or("0");
            let sunrise = day["astronomy"][0]["sunrise"].as_str().unwrap_or("");
            let sunset = day["astronomy"][0]["sunset"].as_str().unwrap_or("");

            tooltip.push_str(&format!("⬆️ {}° ⬇️ {}° ", max_temp, min_temp));
            tooltip.push_str(&format!("🌅 {} 🌆 {}\n", sunrise, sunset));

            // Add hourly forecasts
            if let Some(hourly_array) = day["hourly"].as_array() {
                for hour in hourly_array {
                    if i == 0 {
                        if let Some(time_str) = hour["time"].as_str() {
                            if let Ok(hour_num) = time_str.parse::<i32>() {
                                if hour_num < (current_hour as i32 - 2) {
                                    continue;
                                }
                            }
                        }
                    }

                    let time = hour["time"].as_str().unwrap_or("");
                    let hour_weather_code = hour["weatherCode"].as_str().unwrap_or("113");
                    let hour_feels_like = hour["FeelsLikeC"].as_str().unwrap_or("0");
                    let hour_desc = hour["weatherDesc"][0]["value"].as_str().unwrap_or("");
                    let chances = format_chances(hour);

                    tooltip.push_str(&format!(
                        "{} {} {} {}°, {}\n",
                        format_time(time),
                        get_weather_icon(hour_weather_code),
                        format_temp(hour_feels_like),
                        hour_desc,
                        chances
                    ));
                }
            }
        }
    }

    data.insert("tooltip", tooltip);

    // Output JSON
    println!("{}", serde_json::to_string(&data)?);

    Ok(())
}
