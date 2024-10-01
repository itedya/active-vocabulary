use std::mem::transmute;
use axum::debug_handler;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::extract::Form as ExtractForm;
use axum::response::Html as RespondInHtml;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use sqlx::{Acquire, FromRow, PgPool};
use sqlx::encode::IsNull::No;
use tracing::log::{log, Level};
use crate::components::{header, layout, layout_with_basic_wrappers, add_word_form, InputComponent, InputType, AddWordFormData, AddWordFormErrors, AddWordFormDataRedirectAction};
use crate::html::{ClosableHtmlElement, MultipleHtmlElements, RenderableHtmlElement, Text};
use crate::html::ClosableHtmlElementType::{Button, Div, Form, Main, A, P};

fn log_and_return_internal_error(e: impl std::error::Error) -> Response {
    log!(Level::Error, "{}", e);

    Response::builder()
        .status(500)
        .body("Internal Server Error".into())
        .unwrap()
}

#[derive(Debug, Clone, FromRow)]
struct Word {
    id: i32,
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
    let form = add_word_form(AddWordFormData::default(), AddWordFormErrors::default());

    if headers.contains_key("HX-Request") {
        return Ok(RespondInHtml(form.render()).into_response());
    }

    Ok(RespondInHtml(layout_with_basic_wrappers(form)).into_response())
}

pub async fn add_word(State(pool): State<PgPool>, ExtractForm(form): ExtractForm<AddWordFormData>) -> Result<Response, Response> {
    dbg!(&form);

    let mut errors = AddWordFormErrors::default();

    if form.word.is_empty() {
        errors.word = Some("Word can't be empty".to_string());
    } else if form.word.chars().count() > 200 {
        errors.word = Some("Word can't be longer than 200 characters".to_string());
    }

    if form.translation.is_empty() {
        errors.translation = Some("Translation can't be empty".to_string());
    } else if form.translation.chars().count() > 200 {
        errors.translation = Some("Translation can't be longer than 200 characters".to_string());
    }

    if errors.word.is_some() || errors.translation.is_some() {
        return Ok((StatusCode::OK, RespondInHtml(add_word_form(form, errors).render())).into_response());
    }

    if form.redirect_action == AddWordFormDataRedirectAction::List {
        let mut tx = pool.begin().await.map_err(log_and_return_internal_error)?;

        sqlx::query("INSERT INTO words (word, translation) VALUES ($1, $2)")
            .bind(&form.word)
            .bind(&form.translation)
            .execute(&mut *tx)
            .await
            .map_err(log_and_return_internal_error)?;

        tx.commit().await.map_err(log_and_return_internal_error)?;

        let mut headers = HeaderMap::new();
        headers.insert("HX-Redirect", "/".parse().unwrap());

        return Ok((StatusCode::CREATED, headers).into_response());
    }

    Ok((StatusCode::OK, RespondInHtml(layout_with_basic_wrappers(add_word_form(Default::default(), Default::default())))).into_response())
}