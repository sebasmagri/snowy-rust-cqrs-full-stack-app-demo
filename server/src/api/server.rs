use rocket::{catchers, routes, Build};
use tracing::info;

use crate::api::{cors::CORS, cqrs::setup_cqrs, db::get_db_pool};

use super::{
    config::{get_figment, Config},
    error::Error,
};

pub async fn server(config: Config) -> Result<rocket::Rocket<Build>, Error> {
    info!("initializing...");

    let db_pool = get_db_pool(&config.database_url).await?;
    let cqrs = setup_cqrs(db_pool).await;

    let server = rocket::custom(get_figment())
        .attach(CORS)
        .register(
            "/",
            catchers![
                super::handlers::not_found,
                super::handlers::internal_error,
                super::handlers::bad_request
            ],
        )
        .mount(
            "/",
            routes![
                super::handlers::health,
                super::handlers::command_handler,
                super::handlers::query_handler
            ],
        )
        .manage(config)
        .manage(cqrs);

    info!("successfully initialized!");

    Ok(server)
}
