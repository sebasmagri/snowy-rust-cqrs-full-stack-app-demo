use std::collections::HashMap;

use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};

use snowy_model::{MemberId, WeatherForecast};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub(crate) enum TeamEvent {
    ForecastTracked {
        forecasts: HashMap<MemberId, WeatherForecast>,
    },
    MemberAdded {
        member_id: MemberId,
        email: String,
    },
}

impl DomainEvent for TeamEvent {
    fn event_type(&self) -> String {
        let event_type: &str = match self {
            TeamEvent::ForecastTracked { .. } => "forecast-tracked",
            TeamEvent::MemberAdded { .. } => "member-added",
        };
        event_type.to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}
