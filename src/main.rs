mod handlers;
mod html;
mod components;
mod models;
mod generation_queue;
mod gpt;

use std::sync::{Arc, Mutex};
use std::time::Duration;
use rust_embed::RustEmbed;
use axum::Router;
use axum::routing::{get, post};
use axum_embed::ServeEmbed;
use lazy_static::lazy_static;
use sqlx::postgres::PgPoolOptions;
use tokio_util::sync::CancellationToken;
use crate::generation_queue::generation_queue_worker;

#[derive(RustEmbed, Clone)]
#[folder = "assets/"]
struct Assets;

lazy_static! {
    pub static ref STYLES_CSS_MODTIME: u64 = {
        let asset = Assets::get("styles.css").unwrap();
        let metadata = asset.metadata.last_modified().unwrap();

        metadata
    };
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://av_user:av_password@localhost:8020/av_db".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .unwrap();

    let cancellation_token = CancellationToken::new();

    tokio::spawn(generation_queue_worker(cancellation_token.clone(), pool.clone()));

    let serve_assets = ServeEmbed::<Assets>::new();

    let app = Router::new()
        .route("/", get(handlers::root))
        .route("/add-word", get(handlers::add_word_page).post(handlers::add_word))
        .route("/delete-word", post(handlers::delete_word))
        .route("/teach", get(handlers::teach))
        .nest_service("/assets", serve_assets)
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let webserver_future = axum::serve(listener, app);

    tokio::select! {
        _ = webserver_future => {
            cancellation_token.cancel();
        },
        _ = tokio::signal::ctrl_c() => {
            cancellation_token.cancel();
        }
    }
}
