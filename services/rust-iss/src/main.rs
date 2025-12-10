use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

mod config;
mod domain;
mod handlers;
mod repo;
mod routes;
mod services;

use crate::repo::db::init_db;
use crate::routes::create_router;
use crate::services::job_service::JobService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup logging
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Setup database connection and run migrations
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");
    let pool = PgPoolOptions::new().max_connections(5).connect(&db_url).await?;
    init_db(&pool).await?;
    info!("Database booted and migrations are up-to-date.");

    // Create shared application state
    let state = config::new(pool.clone()).await;

    // Spawn all background jobs
    let job_service = Arc::new(JobService::new(state.clone()));
    job_service.spawn_all_jobs();
    
    // Create and run the Axum web server
    let app = create_router(state);
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", 3000)).await?;
    info!("rust_iss listening on 0.0.0.0:3000");
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
