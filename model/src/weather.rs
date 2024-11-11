use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Deserialize, Debug, PartialEq, Clone)]
pub struct CelsiusTemperature(pub f32);

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum WindSpeedUnit {
    #[serde(rename = "m/s")]
    MetersPerSecond,
    #[serde(rename = "km/h")]
    KilometersPerHour,
    #[serde(rename = "mph")]
    MilesPerHour,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct WindSpeed {
    value: f32,
    unit: WindSpeedUnit,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum WindDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

/// Weather Conditions
/// Uses the WMO weather code
/// https://open-meteo.com/en/docs for more information
#[derive(Serialize, Default, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum WeatherCode {
    #[default]
    ClearSky = 0,
    MainlyClear = 1,
    PartlyCloudy = 2,
    Overcast = 3,
    Fog = 45,
    DepositingRimeFog = 48,
    LightDrizzle = 51,
    ModerateDrizzle = 53,
    DenseDrizzle = 55,
    LightFreezingDrizzle = 56,
    DenseFreezingDrizzle = 57,
    LightRain = 61,
    ModerateRain = 63,
    HeavyRain = 65,
    LightFreezingRain = 66,
    HeavyFreezingRain = 67,
    LightSnow = 71,
    ModerateSnow = 73,
    HeavySnow = 75,
    SnowGrains = 77,
    LightRainShowers = 80,
    ModerateRainShowers = 81,
    HeavyRainShowers = 82,
    LightSnowShowers = 85,
    HeavySnowShowers = 86,
    Thunderstorm = 95,
    LightHailThunderstorm = 96,
    HeavyHailThunderstorm = 99,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct WeatherForecast {
    pub date: NaiveDate,
    pub minimum_temperature: CelsiusTemperature,
    pub maximum_temperature: CelsiusTemperature,
    pub minimum_apparent_temperature: Option<CelsiusTemperature>,
    pub maximum_apparent_temperature: Option<CelsiusTemperature>,
    pub maximum_wind_speed: Option<WindSpeed>,
    pub dominant_wind_direction: Option<WindDirection>,
    pub weather_code: Option<WeatherCode>,
}

impl Default for WeatherForecast {
    fn default() -> Self {
        Self {
            date: Utc::now().date_naive(),
            minimum_temperature: Default::default(),
            maximum_temperature: Default::default(),
            minimum_apparent_temperature: Default::default(),
            maximum_apparent_temperature: Default::default(),
            maximum_wind_speed: Default::default(),
            dominant_wind_direction: Default::default(),
            weather_code: Default::default(),
        }
    }
}
