use std::{net::SocketAddr, sync::Arc, time::Duration};

use anyhow::{Ok, Result};
use axum::{Router, http::Method, routing::get};

use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    limit::{self, RequestBodyLimitLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::info;
use tracing_subscriber::layer;

use crate::{
    config::config_model::DotenvConfig,
    infrastructure::{axum_http::default_routers, postgres::postgres_connection::PgpoolSquad},
};

pub async fn start(config: Arc<DotenvConfig>, db_pool: Arc<PgpoolSquad>) -> Result<()> {
    let app = Router::new()
        .fallback(default_routers::not_found)
        .route("/health-check", get(default_routers::health_check))
        .layer(TimeoutLayer::new(Duration::from_secs(
            config.server.timeout,
        )))
        .layer(RequestBodyLimitLayer::new(
            (config.server.body_limit * 1024 * 1024).try_into()?,
        ))
        .layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_origin(Any),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));

    let lisetener = TcpListener::bind(addr).await?;

    info!("Starting HTTP server on {}", addr);


    axum::serve(lisetener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    

    Ok(())
}

 async fn shutdown_signal()  {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("Signal received, starting graceful shutdown"),
        _ = terminate => info!("Terminate signal received, starting graceful shutdown"),
    }
}
