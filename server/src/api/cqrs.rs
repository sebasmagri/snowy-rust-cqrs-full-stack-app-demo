use std::sync::Arc;

use postgres_es::{postgres_cqrs, PostgresCqrs};
use sqlx::{Pool, Postgres};

use crate::{
    domain::{aggregates::Team, services::TeamServices},
    queries::team::{TeamQuery, TeamQueryDyn, TeamViewRepository},
};

#[derive(Clone)]
pub(crate) struct CqrsPlumbing {
    pub(crate) cqrs: Arc<PostgresCqrs<Team>>,
    pub(crate) team_view_repository: Arc<TeamViewRepository>,
}

pub(crate) async fn setup_cqrs(pool: Pool<Postgres>) -> CqrsPlumbing {
    let team_view_repository = Arc::new(TeamViewRepository::new("team_query", pool.clone()));

    let mut team_query = TeamQuery::new(team_view_repository.clone());
    team_query.use_error_handler(Box::new(|e| {
        eprintln!("Team Query Error: {:?}", e);
    }));

    let queries: Vec<Box<TeamQueryDyn>> = vec![Box::new(team_query)];

    let cqrs = Arc::new(postgres_cqrs(pool, queries, TeamServices {}));

    CqrsPlumbing {
        cqrs,
        team_view_repository,
    }
}
