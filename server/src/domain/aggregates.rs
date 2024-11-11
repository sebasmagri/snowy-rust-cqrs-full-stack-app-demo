use std::{collections::HashMap, vec};

use async_trait::async_trait;
use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};

use snowy_model::{Member, MemberId, Team as TeamModel, WeatherForecast};

use super::{commands::TeamCommand, error::Error, events::TeamEvent, services::TeamServices};

#[derive(Serialize, Debug, Default, Deserialize)]
pub(crate) struct Team {
    pub(crate) team: TeamModel,
    pub(crate) members: Vec<Member>,
    pub(crate) forecasts: HashMap<MemberId, WeatherForecast>,
}

#[async_trait]
impl Aggregate for Team {
    type Command = TeamCommand;
    type Event = TeamEvent;
    type Error = Error;
    type Services = TeamServices;

    fn aggregate_type() -> String {
        "team".to_string()
    }

    async fn handle(
        &self,
        command: Self::Command,
        _services: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            TeamCommand::AddMember { member_id, email } => {
                if self
                    .members
                    .iter()
                    .any(|m| m.email == email || m.id == member_id)
                {
                    return Err(Error::MemberAlreadyExists(email));
                }

                let member = Member::new(member_id, email);
                let event = TeamEvent::MemberAdded {
                    member_id: member.id,
                    email: member.email,
                };
                Ok(vec![event])
            }
            TeamCommand::TrackMemberForecast {
                member_id,
                forecast,
            } => {
                if !self.members.iter().any(|m| m.id == member_id) {
                    return Err(Error::MemberNotFoundInTeam(member_id, self.team.id.clone()));
                }

                let mut new_forecasts = self.forecasts.clone();

                new_forecasts
                    .entry(member_id.clone())
                    .and_modify(|e| {
                        *e = forecast.clone();
                    })
                    .or_insert_with(|| forecast.clone());

                let forecast_tracked_event = TeamEvent::ForecastTracked {
                    forecasts: new_forecasts,
                };

                Ok(vec![forecast_tracked_event])
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            TeamEvent::MemberAdded { member_id, email } => {
                let member = Member::new(member_id, email);
                self.members.push(member);
            }
            TeamEvent::ForecastTracked { forecasts } => {
                self.forecasts = forecasts;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use snowy_model::{CelsiusTemperature, Member, MemberId, WeatherCode, WeatherForecast};

    use cqrs_es::test::TestFramework;

    type TeamTestFramework = TestFramework<Team>;

    #[test]
    fn test_add_member() {
        let email = "test@example.com".to_string();
        let member_id = MemberId::new(uuid::Uuid::new_v4().to_string());
        let command = TeamCommand::AddMember {
            member_id: member_id.clone(),
            email: email.clone(),
        };
        let expected_event = TeamEvent::MemberAdded { member_id, email };

        TeamTestFramework::with(TeamServices)
            .given_no_previous_events()
            .when(command)
            .then_expect_events(vec![expected_event]);
    }

    #[test]
    fn test_add_existing_member() {
        let email = "test@example.com".to_string();
        let member_id = MemberId::new(uuid::Uuid::new_v4().to_string());
        let command = TeamCommand::AddMember {
            member_id: member_id.clone(),
            email: email.clone(),
        };

        TeamTestFramework::with(TeamServices)
            .given(vec![TeamEvent::MemberAdded {
                member_id: member_id.clone(),
                email: email.clone(),
            }])
            .when(command)
            .then_expect_error(Error::MemberAlreadyExists(email));
    }

    #[test]
    fn test_track_first_member_forecast() {
        let member_id = MemberId::new(uuid::Uuid::new_v4().to_string());
        let email = "test@example.com".to_string();
        let member_added_event = TeamEvent::MemberAdded {
            member_id: member_id.clone(),
            email,
        };

        let forecast = WeatherForecast {
            minimum_temperature: CelsiusTemperature(10.0),
            maximum_temperature: CelsiusTemperature(20.0),
            weather_code: Some(WeatherCode::ClearSky),
            ..Default::default()
        };

        let track_forecast_command = TeamCommand::TrackMemberForecast {
            member_id: member_id.clone(),
            forecast: forecast.clone(),
        };
        let expected_events = vec![TeamEvent::ForecastTracked {
            forecasts: [(member_id.clone(), forecast.clone())]
                .iter()
                .cloned()
                .collect(),
        }];

        TestFramework::<Team>::with(TeamServices)
            .given(vec![member_added_event])
            .when(track_forecast_command)
            .then_expect_events(expected_events);
    }

    #[test]
    fn test_apply_member_added() {
        let mut team = Team::default();
        let member_id = MemberId::new(uuid::Uuid::new_v4().to_string());
        let email = "test@example.com".to_string();
        let event = TeamEvent::MemberAdded {
            member_id: member_id.clone(),
            email: email.clone(),
        };

        team.apply(event);

        assert_eq!(team.members.len(), 1);
        assert_eq!(team.members[0].id, member_id);
        assert_eq!(team.members[0].email, email);
    }

    #[test]
    fn test_apply_many_member_added() {
        let mut team = Team::default();

        let events = vec![
            TeamEvent::MemberAdded {
                member_id: MemberId::new(uuid::Uuid::new_v4().to_string()),
                email: "test0@example.com".to_string(),
            },
            TeamEvent::MemberAdded {
                member_id: MemberId::new(uuid::Uuid::new_v4().to_string()),
                email: "test1@example.com".to_string(),
            },
            TeamEvent::MemberAdded {
                member_id: MemberId::new(uuid::Uuid::new_v4().to_string()),
                email: "test2@example.com".to_string(),
            },
        ];

        for event in events {
            team.apply(event);
        }

        assert_eq!(team.members.len(), 3);
        assert_eq!(team.members[0].email, "test0@example.com");
        assert_eq!(team.members[1].email, "test1@example.com");
        assert_eq!(team.members[2].email, "test2@example.com");
    }

    #[test]
    fn test_apply_weather_forecast_tracked() {
        let member_id = MemberId::new(uuid::Uuid::new_v4().to_string());
        let mut team = Team::default();
        team.members.push(Member::new(
            member_id.clone(),
            "test@example.com".to_string(),
        ));

        let forecast = WeatherForecast {
            minimum_temperature: CelsiusTemperature(10.0),
            maximum_temperature: CelsiusTemperature(20.0),
            weather_code: Some(WeatherCode::ClearSky),
            ..Default::default()
        };
        let event = TeamEvent::ForecastTracked {
            forecasts: [(member_id.clone(), forecast.clone())]
                .iter()
                .cloned()
                .collect(),
        };

        team.apply(event);

        assert_eq!(team.forecasts.len(), 1);
        assert_eq!(team.forecasts.get(&member_id).unwrap(), &forecast);
    }

    #[tokio::test]
    async fn test_apply_many_weather_forecast_tracked() {
        // given a team with 4 members
        let member0_id = MemberId::new(uuid::Uuid::new_v4().to_string());
        let member0_email = "test0@example.com";
        let member1_id = MemberId::new(uuid::Uuid::new_v4().to_string());
        let member1_email = "test1@example.com";
        let member2_id = MemberId::new(uuid::Uuid::new_v4().to_string());
        let member2_email = "test2@example.com";
        let member3_id = MemberId::new(uuid::Uuid::new_v4().to_string());
        let member3_email = "test3@example.com";

        let mut team = Team::default();
        team.members.extend(vec![
            Member::new(member0_id.clone(), member0_email.to_string()),
            Member::new(member1_id.clone(), member1_email.to_string()),
            Member::new(member2_id.clone(), member2_email.to_string()),
            Member::new(member3_id.clone(), member3_email.to_string()),
        ]);

        let service = TeamServices {};

        // when a forecast is tracked for each member
        let member0_forecast0 = WeatherForecast {
            minimum_temperature: CelsiusTemperature(10.0),
            maximum_temperature: CelsiusTemperature(20.0),
            weather_code: Some(WeatherCode::ClearSky),
            ..Default::default()
        };
        let member0_forecast0_command = TeamCommand::TrackMemberForecast {
            member_id: member0_id.clone(),
            forecast: member0_forecast0.clone(),
        };
        let member0_forecast0_events = team
            .handle(member0_forecast0_command, &service)
            .await
            .unwrap();

        for event in member0_forecast0_events {
            team.apply(event);
        }

        // check after first forecast
        assert_eq!(team.forecasts.len(), 1);
        assert_eq!(team.forecasts.get(&member0_id).unwrap(), &member0_forecast0);

        // another forecast for the same member
        let member0_forecast1 = WeatherForecast {
            minimum_temperature: CelsiusTemperature(8.0),
            maximum_temperature: CelsiusTemperature(12.0),
            weather_code: Some(WeatherCode::ModerateRain),
            ..Default::default()
        };
        let member0_forecast1_command = TeamCommand::TrackMemberForecast {
            member_id: member0_id.clone(),
            forecast: member0_forecast1.clone(),
        };
        let member0_forecast1_events = team
            .handle(member0_forecast1_command, &service)
            .await
            .unwrap();

        for event in member0_forecast1_events {
            team.apply(event);
        }

        // check after another forecast for the same member
        assert_eq!(team.forecasts.len(), 1);
        assert_eq!(team.forecasts.get(&member0_id).unwrap(), &member0_forecast1);

        // forecast for another member
        let member1_forecast0 = WeatherForecast {
            minimum_temperature: CelsiusTemperature(15.0),
            maximum_temperature: CelsiusTemperature(25.0),
            weather_code: Some(WeatherCode::MainlyClear),
            ..Default::default()
        };
        let member1_forecast0_command = TeamCommand::TrackMemberForecast {
            member_id: member1_id.clone(),
            forecast: member1_forecast0.clone(),
        };
        let member1_forecast0_events = team
            .handle(member1_forecast0_command, &service)
            .await
            .unwrap();
        for event in member1_forecast0_events {
            team.apply(event);
        }

        // check after forecast for another member
        assert_eq!(team.forecasts.len(), 2);
        assert_eq!(team.forecasts.get(&member1_id).unwrap(), &member1_forecast0);
    }
}
