use rand::random;
use crate::html::{ClosableHtmlElement, MultipleHtmlElements, NonClosableHtmlElement, RenderableHtmlElement, Text};
use crate::html::ClosableHtmlElementType::{Body, Div, Head, Header, Html, Label, Main, Script, Title, A, H1};
use crate::html::NonClosableHtmlElementType::{Doctype, Input, Link, Meta};

pub fn layout(body: impl RenderableHtmlElement) -> String {
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
                                        .add_element(NonClosableHtmlElement::new(Meta).with_attribute("charset", "utf-8"))
                                        .add_element(NonClosableHtmlElement::new(Meta).with_attribute("name", "viewport")
                                            .with_attribute("content", "width=device-width, user-scalable=no, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0"))
                                        .add_element(NonClosableHtmlElement::new(Meta).with_attribute("http-equiv", "X-UA-Compatible")
                                            .with_attribute("content", "IE=edge"))
                                        .add_element(ClosableHtmlElement::new(Title).with_content(Text::new("Hello, World!")))
                                        .add_element(NonClosableHtmlElement::new(Link).with_attribute("rel", "stylesheet")
                                            .with_attribute("href", format!("/assets/styles.css?{}", *crate::STYLES_CSS_MODTIME)))
                                )
                        )
                        .add_element(
                            ClosableHtmlElement::new(Body)
                                .with_content(
                                    MultipleHtmlElements::new()
                                        .add_element(body)
                                        .add_element(
                                            ClosableHtmlElement::new(Script)
                                                .with_attribute("src", "/assets/htmx.2.0.2.min.js")
                                        )
                                )
                        )
                )
        )
        .render()
}

pub fn header() -> impl RenderableHtmlElement {
    ClosableHtmlElement::new(Header)
        .with_attribute("class", "header")
        .with_content(
            MultipleHtmlElements::new()
                .add_element(
                    ClosableHtmlElement::new(A)
                        .with_attribute("class", "nav-title")
                        .with_attribute("href", "/")
                        .with_content(
                            ClosableHtmlElement::new(H1).with_content(Text::new("Active Vocabulary"))
                        ))
                .add_element(
                    ClosableHtmlElement::new(Div).with_attribute("class", "nav-items")
                        .with_content(
                            MultipleHtmlElements::new()
                                .add_element(ClosableHtmlElement::new(A)
                                    .with_attribute("href", "/")
                                    .with_content(Text::new("All words")))
                                .add_element(ClosableHtmlElement::new(A)
                                    .with_attribute("href", "/about")
                                    .with_content(Text::new("About")))
                        )
                )
        )
}

pub fn layout_with_basic_wrappers(body: impl RenderableHtmlElement) -> String {
    layout(
        ClosableHtmlElement::new(Div)
            .with_attribute("class", "container")
            .with_content(
                MultipleHtmlElements::new()
                    .add_element(header())
                    .add_element(ClosableHtmlElement::new(Main)
                        .with_content(body))
            )
    )
}

pub enum InputType {
    Text,
    Number,
    Password,
}

impl Into<String> for InputType {
    fn into(self) -> String {
        match self {
            InputType::Text => "text".to_string(),
            InputType::Number => "number".to_string(),
            InputType::Password => "password".to_string(),
        }
    }
}

pub fn input(r#type: InputType, label: impl Into<String>, name: impl Into<String>, custom_id: Option<impl Into<String>>) -> impl RenderableHtmlElement {
    let type_stringified: String = r#type.into();

    let label = label.into();
    let name = name.into();

    let id = custom_id.map_or_else(|| {
        let randomizer = random::<u64>();
        let name = name.clone();

        format!("{}-{}", name, randomizer)
    }, |v| {
        v.into()
    });

    ClosableHtmlElement::new(Div)
        .with_attribute("class", "input-wrapper")
        .with_content(
            MultipleHtmlElements::new()
                .add_element(
                    ClosableHtmlElement::new(Label)
                        .with_attribute("for", id.clone())
                        .with_content(Text::new(label))
                )
                .add_element(
                    NonClosableHtmlElement::new(Input)
                        .with_attribute("type", type_stringified)
                        .with_attribute("name", name)
                        .with_attribute("id", id)
                )
        )
}