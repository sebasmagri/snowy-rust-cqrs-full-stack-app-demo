use std::io::Cursor;

use cqrs_es::AggregateError;
use rocket::{http::ContentType, response::Responder, Response};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Internal error: {0}")]
    Aggregate(#[from] AggregateError<crate::domain::error::Error>),
    #[error("Internal error: {0}")]
    View(#[from] cqrs_es::persist::PersistenceError),
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let err_body = json!({ "error": format!("{:?}", &self) }).to_string();
        Response::build()
            .header(ContentType::JSON)
            .sized_body(err_body.len(), Cursor::new(err_body))
            .ok()
    }
}
