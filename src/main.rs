use std::sync::Arc;

use quests_tracker::{config::{config_loader}, infrastructure::{axum_http::http_serve::start, postgres::postgres_connection}};
use tracing::{error, info};


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dotenvy_env = match config_loader::load() {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };

    info!("Env has been loaded successfully");


    let postgres_pool = match postgres_connection::estabish_connection_pool(&dotenvy_env.database.url) {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to create Postgres connection pool: {}", e);
            std::process::exit(1);
        }
    };

    info!("Postgres connection pool established successfully");

    start(Arc::new(dotenvy_env), Arc::new(postgres_pool))
        .await
        .expect("Failed to start the HTTP server");

}
