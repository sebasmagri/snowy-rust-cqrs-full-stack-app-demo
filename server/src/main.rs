use tracing::info;

mod api;
pub(crate) mod domain;
mod queries;

fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();
    let config = api::config::get_config();
    ::rocket::async_main(async move {
        info!(config = %config, "starting up server");
        let _res = api::server::server(config)
            .await
            .expect("Failed to launch server")
            .launch()
            .await;
    });

    Ok(())
}
