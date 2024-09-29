use axum::debug_handler;
use axum::extract::State;
use axum::response::Html as RespondInHtml;
use axum::response::{IntoResponse, Response};
use sqlx::{FromRow, PgPool};
use tracing::log::{log, Level};
use crate::components::{header, layout};
use crate::html::{ClosableHtmlElement, MultipleHtmlElements};
use crate::html::ClosableHtmlElementType::{Div, Main};

fn log_and_return_internal_error(e: impl std::error::Error) -> Response {
    log!(Level::Error, "{}", e);

    Response::builder()
        .status(500)
        .body("Internal Server Error".into())
        .unwrap()
}

#[derive(Debug, Clone, FromRow)]
struct Word {
    id: i64,
    word: String,
    translation: String,
}

#[debug_handler]
pub async fn root(State(pool): State<PgPool>) -> Result<Response, Response> {
    let mut tx = pool.begin().await.map_err(log_and_return_internal_error)?;

    let words: Vec<Word> = sqlx::query_as("SELECT * FROM words")
        .fetch_all(&mut *tx)
        .await
        .map_err(log_and_return_internal_error)?;

    tx.commit().await.map_err(log_and_return_internal_error)?;

    log!(Level::Info, "{:?}", words);

    Ok(RespondInHtml(layout(
        ClosableHtmlElement::new(Div)
            .with_attribute("class", "container")
            .with_content(
                MultipleHtmlElements::new()
                    .add_element(header())
                    .add_element(ClosableHtmlElement::new(Main))
            )
    )).into_response())
}