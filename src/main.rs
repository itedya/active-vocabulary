mod handlers;
mod html;

use rust_embed::RustEmbed;
use axum::Router;
use axum::routing::get;
use axum_embed::ServeEmbed;
use lazy_static::lazy_static;

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

    let serve_assets = ServeEmbed::<Assets>::new();

    let app = Router::new()
        .route("/", get(handlers::root))
        .nest_service("/assets", serve_assets);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
