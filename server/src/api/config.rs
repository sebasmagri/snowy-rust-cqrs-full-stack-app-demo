use std::fmt::Display;

use rocket::figment::{
    providers::{Env, Serialized},
    Figment,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct Config {
    pub(crate) database_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: "postgres://localhost/snowy".to_string(),
        }
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Config {}",
            serde_json::to_string(self).expect("Failed to serialize Config")
        )
    }
}

pub(crate) fn get_figment() -> Figment {
    rocket::Config::figment()
        .merge(Serialized::defaults(Config::default()))
        .merge(Env::prefixed("SNOWY_").global())
}

pub(crate) fn get_config() -> Config {
    get_figment().extract().expect("Configuration error")
}
