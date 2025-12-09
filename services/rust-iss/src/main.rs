use sqlx::postgres::PgPoolOptions;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

mod config;
use crate::config;
mod handlers;
mod routes;
mod clients;
mod repo;

use crate::repo::db::init_db;
use crate::clients::{iss, nasa, osdr, spacex};
use crate::routes::create_router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");
    let pool = PgPoolOptions::new().max_connections(5).connect(&db_url).await?;
    init_db(&pool).await?;
    info!("database booted");

    let state = config::new(pool.clone()).await;

    // фон OSDR
    {
        let st = state.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = osdr::fetch_and_store_osdr(&st).await { error!("osdr err {e:?}") }
                tokio::time::sleep(tokio::time::Duration::from_secs(st.every_osdr)).await;
            }
        });
    }
    // фон ISS
    {
        let st = state.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = iss::fetch_and_store_iss(&st.pool, &st.iss_url).await { error!("iss err {e:?}") }
                tokio::time::sleep(tokio::time::Duration::from_secs(st.every_iss)).await;
            }
        });
    }
    // фон APOD
    {
        let st = state.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = nasa::fetch_apod(&st).await { error!("apod err {e:?}") }
                tokio::time::sleep(tokio::time::Duration::from_secs(st.every_apod)).await;
            }
        });
    }
    // фон NeoWs
    {
        let st = state.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = nasa::fetch_neo_feed(&st).await { error!("neo err {e:?}") }
                tokio::time::sleep(tokio::time::Duration::from_secs(st.every_neo)).await;
            }
        });
    }
    // фон DONKI
    {
        let st = state.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = nasa::fetch_donki(&st).await { error!("donki err {e:?}") }
                tokio::time::sleep(tokio::time::Duration::from_secs(st.every_donki)).await;
            }
        });
    }
    // фон SpaceX
    {
        let st = state.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = spacex::fetch_spacex_next(&st).await { error!("spacex err {e:?}") }
                tokio::time::sleep(tokio::time::Duration::from_secs(st.every_spacex)).await;
            }
        });
    }

    let app = routes::create_router(state);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", 3000)).await?;
    info!("rust_iss listening on 0.0.0.0:3000");
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

/* ---------- Фетчеры и запись ---------- */





