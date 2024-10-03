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
use crate::components::{header, layout, layout_with_basic_wrappers, add_word_form, InputComponent, InputType, AddWordFormData, AddWordFormErrors, AddWordFormDataRedirectAction, word_list_component};
use crate::html::{ClosableHtmlElement, MultipleHtmlElements, RenderableHtmlElement, Text, UnsafeText};
use crate::html::ClosableHtmlElementType::{Button, Div, Form, Head, Main, Table, Tbody, Td, Th, Thead, Tr, A, P};
use crate::models::Word;

fn log_and_return_internal_error(e: impl std::error::Error) -> Response {
    log!(Level::Error, "{}", e);

    Response::builder()
        .status(500)
        .body("Internal Server Error".into())
        .unwrap()
}

#[debug_handler]
pub async fn root(State(pool): State<PgPool>) -> Result<Response, Response> {
    let mut tx = pool.begin().await.map_err(log_and_return_internal_error)?;

    let words: Vec<Word> = sqlx::query_as("SELECT * FROM words")
        .fetch_all(&mut *tx)
        .await.map_err(log_and_return_internal_error)?;

    tx.commit().await.map_err(log_and_return_internal_error)?;

    Ok(RespondInHtml(layout_with_basic_wrappers(word_list_component(words))).into_response())
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

    let mut tx = pool.begin().await.map_err(log_and_return_internal_error)?;

    sqlx::query("INSERT INTO words (word, translation) VALUES ($1, $2)")
        .bind(form.word)
        .bind(form.translation)
        .execute(&mut *tx)
        .await.map_err(log_and_return_internal_error)?;

    if form.redirect_action == AddWordFormDataRedirectAction::List {
        let words: Vec<Word> = sqlx::query_as("SELECT * FROM words")
            .fetch_all(&mut *tx)
            .await.map_err(log_and_return_internal_error)?;

        tx.commit().await.map_err(log_and_return_internal_error)?;

        let mut headers = HeaderMap::new();
        headers.insert("HX-Push-Url", "/".parse().unwrap());
        headers.insert("HX-Retarget", "body".parse().unwrap());

        return Ok((StatusCode::OK, headers, RespondInHtml(layout_with_basic_wrappers(word_list_component(words)))).into_response());
    }

    tx.commit().await.map_err(log_and_return_internal_error)?;

    let mut headers = HeaderMap::new();
    headers.insert("HX-Retarget", "body".parse().unwrap());

    Ok((StatusCode::OK, headers, RespondInHtml(layout_with_basic_wrappers(add_word_form(Default::default(), Default::default())))).into_response())
}

#[derive(Debug, Deserialize)]
pub struct DeleteWordForm {
    id: i32,
}

#[debug_handler]
pub async fn delete_word(State(pool): State<PgPool>, ExtractForm(form): ExtractForm<DeleteWordForm>) -> Result<Response, Response> {
    let mut tx = pool.begin().await.map_err(log_and_return_internal_error)?;

    sqlx::query("DELETE FROM words WHERE id = $1")
        .bind(form.id)
        .execute(&mut *tx)
        .await
        .map_err(log_and_return_internal_error)?;

    let words: Vec<Word> = sqlx::query_as("SELECT * FROM words")
        .fetch_all(&mut *tx)
        .await
        .map_err(log_and_return_internal_error)?;

    tx.commit().await.map_err(log_and_return_internal_error)?;

    let response_html = layout_with_basic_wrappers(word_list_component(words));

    let mut headers = HeaderMap::new();
    headers.insert("HX-Retarget", "body".parse().unwrap());

    Ok((StatusCode::OK, headers, RespondInHtml(response_html)).into_response())
}

#[debug_handler]
pub async fn teach(State(pool): State<PgPool>, request_headers: HeaderMap) -> Result<Response, Response> {
    let mut tx = pool.begin().await.map_err(log_and_return_internal_error)?;

    let lacking_examples_and_not_in_process_of_generation: Vec<i32> = sqlx::query_scalar(
        concat!("SELECT words.id FROM words ",
        "LEFT JOIN examples ON words.id = examples.word_id ",
        "LEFT JOIN example_generation_queue ON words.id = example_generation_queue.word_id WHERE examples.id IS NULL AND example_generation_queue.id IS NULL"
        )
    )
        .fetch_all(&mut *tx)
        .await
        .map_err(log_and_return_internal_error)?;

    // add them to generation if none are in process of generation
    if !lacking_examples_and_not_in_process_of_generation.is_empty() {
        dbg!(lacking_examples_and_not_in_process_of_generation.clone());

        let ids = lacking_examples_and_not_in_process_of_generation.iter()
            .map(|id| id.to_string()).collect::<Vec<String>>().join(", ");

        // this sql injection in safe, because we're the ones who generate the ids
        sqlx::query(&format!("INSERT INTO example_generation_queue (word_id) VALUES ({})", ids))
            .bind(ids)
            .execute(&mut *tx)
            .await
            .map_err(log_and_return_internal_error)?;
    }

    let in_generation: Vec<Word> = sqlx::query_as("SELECT words.* FROM example_generation_queue INNER JOIN words ON example_generation_queue.word_id = words.id")
        .fetch_all(&mut *tx)
        .await
        .map_err(log_and_return_internal_error)?;

    tx.commit().await.map_err(log_and_return_internal_error)?;

    if !in_generation.is_empty() {
        let content = MultipleHtmlElements::new()
            .add_element(Text::new(format!("Generating examples for words: {}. Please wait.", in_generation.iter().map(|word| word.word.clone()).collect::<Vec<String>>().join(", "))))
            .add_element(UnsafeText::new("<script>setTimeout(() => { window.location.reload(); }, 3000);</script>"));

        if request_headers.contains_key("HX-Request") {
            return Ok((StatusCode::OK, RespondInHtml(content.render())).into_response());
        }

        return Ok((StatusCode::OK, RespondInHtml(layout_with_basic_wrappers(content))).into_response());
    }

    // Ok((StatusCode::OK, RespondInHtml(layout_with_basic_wrappers(Text::new("Teach page")))).into_response());
    unimplemented!()
}