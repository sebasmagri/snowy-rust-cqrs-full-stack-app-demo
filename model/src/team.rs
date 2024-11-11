use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{CelsiusTemperature, WeatherCode, WeatherForecast};

#[derive(Serialize, Default, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(transparent)]
pub struct MemberId(String);

impl MemberId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

#[derive(Serialize, Default, Deserialize, Debug, PartialEq, Clone)]
pub struct Member {
    pub id: MemberId,
    pub email: String,
}

impl Member {
    pub fn new(id: MemberId, email: String) -> Self {
        Self { id, email }
    }
}

#[derive(Serialize, Default, Deserialize, Debug, PartialEq, Clone)]
#[serde(transparent)]
pub struct TeamId(String);

impl From<String> for TeamId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for TeamId {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

#[derive(Serialize, Default, Deserialize, Debug, PartialEq, Clone)]
pub struct Team {
    pub id: TeamId,
    pub name: String,
    pub roles: Vec<String>,
}

// for queries
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct TeamView {
    pub id: TeamId,
    pub name: String,
    pub members: Vec<Member>,
    pub forecasts: HashMap<MemberId, WeatherForecast>,
    pub total_forecasts_tracked: u64,
    pub avg_minimum_temperature: Option<CelsiusTemperature>,
    pub avg_maximum_temperature: Option<CelsiusTemperature>,
    pub weather_condition_distribution: HashMap<WeatherCode, i32>,
}
