use std::collections::HashMap;

use cqrs_es::Query;
use cqrs_es::{persist::GenericQuery, EventEnvelope, View};
use postgres_es::PostgresViewRepository;

use snowy_model::{CelsiusTemperature, Member, TeamView};

use crate::domain::aggregates::Team;
use crate::domain::events::TeamEvent;

pub(crate) type TeamViewRepository = PostgresViewRepository<TeamView, Team>;
pub(crate) type TeamQuery = GenericQuery<TeamViewRepository, TeamView, Team>;
pub(crate) type TeamQueryDyn = dyn Query<Team>;

impl View<Team> for TeamView {
    fn update(&mut self, event: &EventEnvelope<Team>) {
        match &event.payload {
            TeamEvent::MemberAdded { member_id, email } => {
                self.members.push(Member {
                    id: member_id.clone(),
                    email: email.clone(),
                });
            }
            TeamEvent::ForecastTracked { forecasts } => {
                let n_forecasts = forecasts.len() as f32;

                let avg_minimum_temperature = CelsiusTemperature(
                    forecasts
                        .values()
                        .map(|f| f.minimum_temperature.0)
                        .sum::<f32>()
                        / n_forecasts,
                );
                let avg_maximum_temperature = CelsiusTemperature(
                    forecasts
                        .values()
                        .map(|f| f.maximum_temperature.0)
                        .sum::<f32>()
                        / n_forecasts,
                );

                let weather_condition_distribution =
                    forecasts.values().fold(HashMap::new(), |mut acc, f| {
                        if let Some(wc) = &f.weather_code {
                            *acc.entry(wc.clone()).or_insert(0) += 1;
                        }
                        acc
                    });

                self.forecasts = forecasts.clone();
                self.avg_maximum_temperature = Some(avg_maximum_temperature);
                self.avg_minimum_temperature = Some(avg_minimum_temperature);
                self.weather_condition_distribution = weather_condition_distribution;
                self.total_forecasts_tracked += 1;
            }
        }
    }
}
