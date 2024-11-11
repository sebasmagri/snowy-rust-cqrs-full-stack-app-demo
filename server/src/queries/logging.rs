use async_trait::async_trait;

use cqrs_es::{EventEnvelope, Query};

use crate::domain::aggregates::Team;

/// Dummy logging query that prints all events to the console for testing purposes.
pub(crate) struct EventLoggingQuery {}

#[async_trait]
impl Query<Team> for EventLoggingQuery {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<Team>]) {
        for event in events {
            println!(
                "TeamEvent({}, {}):\n{}",
                aggregate_id,
                event.sequence,
                serde_json::to_string_pretty(&event.payload).unwrap()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use cqrs_es::{mem_store::MemStore, CqrsFramework};

    use super::*;
    use crate::domain::{commands::TeamCommand, services::TeamServices};
    use snowy_model::{CelsiusTemperature, MemberId, WeatherCode, WeatherForecast};

    #[tokio::test]
    async fn test_event_logging_query() {
        let event_store = MemStore::<Team>::default();
        let query = EventLoggingQuery {};
        let cqrs = CqrsFramework::new(event_store, vec![Box::new(query)], TeamServices {});

        let aggregate_id = "team-1";

        cqrs.execute(
            aggregate_id,
            TeamCommand::AddMember {
                member_id: MemberId::new("member-1".to_string()),
                email: "test@example.com".to_string(),
            },
        )
        .await
        .unwrap();

        cqrs.execute(
            aggregate_id,
            TeamCommand::TrackMemberForecast {
                member_id: MemberId::new("member-1".to_string()),
                forecast: WeatherForecast {
                    weather_code: Some(WeatherCode::Fog),
                    minimum_temperature: CelsiusTemperature(10.0),
                    maximum_temperature: CelsiusTemperature(20.0),
                    ..Default::default()
                },
            },
        )
        .await
        .unwrap();

        cqrs.execute(
            aggregate_id,
            TeamCommand::AddMember {
                member_id: MemberId::new("member-2".to_string()),
                email: "test2@example.com".to_string(),
            },
        )
        .await
        .unwrap();

        cqrs.execute(
            aggregate_id,
            TeamCommand::TrackMemberForecast {
                member_id: MemberId::new("member-2".to_string()),
                forecast: WeatherForecast {
                    weather_code: Some(WeatherCode::DenseDrizzle),
                    minimum_temperature: CelsiusTemperature(5.0),
                    maximum_temperature: CelsiusTemperature(12.0),
                    ..Default::default()
                },
            },
        )
        .await
        .unwrap();
    }
}
