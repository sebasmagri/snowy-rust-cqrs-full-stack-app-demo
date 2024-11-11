use rocket::{
    catch, get, post,
    request::Request,
    serde::json::{self, json, Value},
    State,
};

use super::{cqrs::CqrsPlumbing, error::Error};
use cqrs_es::persist::ViewRepository; // FIXME: move over

#[get("/health")]
pub fn health() -> Value {
    json!({
        "status": "ok"
    })
}

#[catch(400)]
pub fn bad_request(_req: &Request) -> Value {
    json!({
        "error": "Bad request"
    })
}

#[catch(404)]
pub fn not_found(_req: &Request) -> Value {
    json!({
        "error": "Not found"
    })
}

#[catch(500)]
pub fn internal_error(req: &Request) -> Value {
    json!({
        "error": "Internal error",
        "uri": format!("{}", req.uri())
    })
}

#[post("/api/team/<team_id>", data = "<payload>")]
pub async fn command_handler(
    cqrs: &State<CqrsPlumbing>,
    team_id: &str,
    payload: json::Json<crate::domain::commands::TeamCommand>,
) -> Result<Value, Error> {
    cqrs.cqrs.execute(team_id, payload.0.clone()).await?;
    Ok(json!({
        "status": "ok"
    }))
}

#[get("/api/team/<team_id>")]
pub async fn query_handler(
    cqrs: &State<CqrsPlumbing>,
    team_id: &str,
) -> Result<Option<Value>, Error> {
    match cqrs.team_view_repository.load(team_id).await? {
        Some(team_view) => Ok(Some(json!(team_view))),
        None => Ok(None),
    }
}
