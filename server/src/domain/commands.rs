use serde::{Deserialize, Serialize};

use snowy_model::{MemberId, WeatherForecast};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) enum TeamCommand {
    AddMember {
        member_id: MemberId,
        email: String,
    },
    TrackMemberForecast {
        member_id: MemberId,
        forecast: WeatherForecast,
    },
}
