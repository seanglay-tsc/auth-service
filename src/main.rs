use std::sync::Arc;

use axum::{self};
use config::{DATABASE_URL, HOST, PORT};
use route::create_router;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::net::TcpListener;

mod config;
mod handler;
mod response;
mod route;
mod service;

#[derive(Clone)]
pub struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt().init();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&DATABASE_URL)
        .await
    {
        Ok(pool) => {
            tracing::info!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            tracing::error!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let tcp_listener = TcpListener::bind(&format!("{}:{}", *HOST, *PORT))
        .await
        .expect("Unable to connect to the server");

    tracing::info!("Listening on {}", tcp_listener.local_addr().unwrap());

    let app = create_router(Arc::new(AppState { db: pool }));

    axum::serve(tcp_listener, app)
        .await
        .expect("Server failed to start");
}
