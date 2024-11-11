use super::error::Error;

pub(crate) async fn get_db_pool(database_url: &str) -> Result<sqlx::Pool<sqlx::Postgres>, Error> {
    Ok(sqlx::Pool::connect(database_url).await?)
}
