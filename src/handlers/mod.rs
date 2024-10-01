use axum::debug_handler;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::Html as RespondInHtml;
use axum::response::{IntoResponse, Response};
use sqlx::{FromRow, PgPool};
use tracing::log::{log, Level};
use crate::components::{header, layout, layout_with_basic_wrappers};
use crate::html::{ClosableHtmlElement, MultipleHtmlElements, RenderableHtmlElement, Text};
use crate::html::ClosableHtmlElementType::{Button, Div, Form, Main, A};

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

    Ok(RespondInHtml(layout_with_basic_wrappers(
        MultipleHtmlElements::new()
            .add_element(ClosableHtmlElement::new(A)
                .with_attribute("href", "/add-word")
                .with_attribute("hx-boost", "true")
                .with_attribute("hx-target", "main")
                .with_attribute("class", "add-word-button")
                .with_content(Text::new("Add word")))
            .add_element(
                ClosableHtmlElement::new(P)
                    .with_attribute("class", "no-words-message")
                    .with_content(Text::new("You have no words yet. Add them by clicking the button above."))
            )
    )).into_response())
}

#[debug_handler]
pub async fn add_word_page(headers: HeaderMap) -> Result<Response, Response> {
    let form = ClosableHtmlElement::new(Form)
        .with_attribute("class", "add-word-form")
        .with_content(
            MultipleHtmlElements::new()
                .add_element(ClosableHtmlElement::new(Button)
                    .with_attribute("hx-post", "/add-word")
                    .with_content(Text::new("Add word")))
        );

    if headers.contains_key("HX-Request") {
        return Ok(RespondInHtml(form.render()).into_response());
    }

    Ok(RespondInHtml(layout_with_basic_wrappers(form)).into_response())
}