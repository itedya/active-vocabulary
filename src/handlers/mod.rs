use axum::debug_handler;
use axum::response::Html as RespondInHtml;
use axum::response::{IntoResponse, Response};
use crate::components::{header, layout};
use crate::html::{ClosableHtmlElement, MultipleHtmlElements};
use crate::html::ClosableHtmlElementType::{Div, Main};

#[debug_handler]
pub async fn root() -> Response {
    RespondInHtml(layout(
        ClosableHtmlElement::new(Div)
            .with_attribute("class", "container")
            .with_content(
                MultipleHtmlElements::new()
                    .add_element(header())
                    .add_element(ClosableHtmlElement::new(Main))
            )
    )).into_response()
}