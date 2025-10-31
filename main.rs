use axum::{routing::{get}, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

mod db;
mod models;
mod routes;
mod services;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let pool = db::init_pool().await?;

    // sqlx::migrate!().run(&pool).await?; // enable after sqlx-cli setup

    let api = routes::router(pool.clone());

    let app = Router::new()
        .nest("/api", api)
        .layer(CorsLayer::very_permissive().allow_origin(Any));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("backend listening on http://{addr}");
    axum::Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}
