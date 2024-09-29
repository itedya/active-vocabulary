use axum::debug_handler;
use axum::response::Html as RespondInHtml;
use axum::response::{IntoResponse, Response};
use crate::html::{ClosableHtmlElement, MultipleHtmlElements, NonClosableHtmlElement, NonClosableHtmlElementType, RenderableHtmlElement, Text};
use crate::html::ClosableHtmlElementType::{Body, Head, Html, Title};
use crate::html::NonClosableHtmlElementType::{Doctype, Link, Meta};

fn layout(body: Option<String>) -> String {
    MultipleHtmlElements::new()
        .add_element(NonClosableHtmlElement::new(Doctype))
        .add_element(
            ClosableHtmlElement::new(Html)
                .with_content(
                    MultipleHtmlElements::new()
                        .add_element(
                            ClosableHtmlElement::new(Head)
                                .with_content(
                                    MultipleHtmlElements::new()
                                        .add_element(NonClosableHtmlElement::new(Meta).with_attribute("charset".to_string(), "utf-8".to_string()))
                                        .add_element(NonClosableHtmlElement::new(Meta).with_attribute("name".to_string(), "viewport".to_string())
                                            .with_attribute("content".to_string(), "width=device-width, user-scalable=no, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0".to_string()))
                                        .add_element(NonClosableHtmlElement::new(Meta).with_attribute("http-equiv".to_string(), "X-UA-Compatible".to_string())
                                            .with_attribute("content".to_string(), "IE=edge".to_string()))
                                        .add_element(ClosableHtmlElement::new(Title).with_content(Text::new("Hello, World!")))
                                        .add_element(NonClosableHtmlElement::new(Link).with_attribute("rel".to_string(), "stylesheet".to_string())
                                            .with_attribute("href".to_string(), format!("/assets/styles.css?{}", *crate::STYLES_CSS_MODTIME)))
                                )
                        )
                        .add_element(
                            ClosableHtmlElement::new(Body)
                                .with_content(
                                    Text::new(body.unwrap_or_else(|| "Hello, World!".to_string()))
                                )
                        )
                )
        )
        .render()
}

#[debug_handler]
pub async fn root() -> Response {
    RespondInHtml(layout(None)).into_response()
}