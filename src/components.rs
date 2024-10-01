use rand::random;
use serde::Deserialize;
use crate::html::{ClosableHtmlElement, MultipleHtmlElements, NonClosableHtmlElement, RenderableHtmlElement, Text};
use crate::html::ClosableHtmlElementType::{Body, Button, Div, Form, Head, Header, Html, Label, Main, Script, Title, A, H1};
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
}

impl Into<String> for InputType {
    fn into(self) -> String {
        match self {
            InputType::Text => "text".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum InputComponent<T>
where
    T: Into<String>,
{
    Text {
        label: T,
        name: T,
        custom_id: Option<T>,
        value: T,
        error: Option<T>,
    }
}

impl<T> InputComponent<T>
where
    T: Into<String>,
    String: for<'a> From<&'a T>,
    Option<T>: Clone,
{
    pub fn render(&self) -> impl RenderableHtmlElement + '_ {
        match self {
            InputComponent::Text { label, name, custom_id, value, error } => {
                render_input(InputType::Text, label, name, (*custom_id).clone(), value, (*error).clone())
            }
        }
    }
}

fn render_input(r#type: InputType, label: impl Into<String>, name: impl Into<String>, custom_id: Option<impl Into<String>>, value: impl Into<String>, error: Option<impl Into<String>>) -> impl RenderableHtmlElement {
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

    let mut elements_inside_wrapper = MultipleHtmlElements::new()
        .add_element(
            ClosableHtmlElement::new(Label)
                .with_attribute("for", id.clone())
                .with_content(Text::new(label))
        )
        .add_element(
            NonClosableHtmlElement::new(Input)
                .with_attribute("value", value)
                .with_attribute("type", type_stringified)
                .with_attribute("name", name)
                .with_attribute("id", id)
        );

    if let Some(error) = error {
        elements_inside_wrapper = elements_inside_wrapper.add_element(
            ClosableHtmlElement::new(Div)
                .with_attribute("class", "error")
                .with_content(Text::new(error))
        );
    }

    ClosableHtmlElement::new(Div)
        .with_attribute("class", "input-wrapper")
        .with_content(elements_inside_wrapper)
}

#[derive(Debug, Clone, Deserialize, Default, PartialEq)]
pub enum AddWordFormDataRedirectAction {
    List,
    #[default]
    Stay,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct AddWordFormData {
    pub redirect_action: AddWordFormDataRedirectAction,
    pub word: String,
    pub translation: String,
}

#[derive(Debug, Default)]
pub struct AddWordFormErrors {
    pub word: Option<String>,
    pub translation: Option<String>,
}

pub fn add_word_form(data: AddWordFormData, errors: AddWordFormErrors) -> impl RenderableHtmlElement {
    ClosableHtmlElement::new(Form)
        .with_attribute("class", "add-word-form")
        .with_content(
            MultipleHtmlElements::new()
                .add_element(
                    ClosableHtmlElement::new(Div)
                        .with_attribute("class", "flex flex-col gap-12")
                        .with_content(
                            MultipleHtmlElements::new()
                                .add_element(InputComponent::Text {
                                    label: "Word".to_string(),
                                    name: "word".to_string(),
                                    value: data.word,
                                    error: errors.word,
                                    custom_id: None,
                                }.render())
                                .add_element(InputComponent::Text {
                                    label: "Translation".to_string(),
                                    name: "translation".to_string(),
                                    value: data.translation,
                                    error: errors.translation,
                                    custom_id: None,
                                }.render())
                        )
                )
                .add_element(
                    ClosableHtmlElement::new(Div)
                        .with_attribute("class", "flex-row")
                        .with_content(
                            MultipleHtmlElements::new()
                                .add_element(
                                    ClosableHtmlElement::new(Button)
                                        .with_attribute("hx-post", "/add-word")
                                        .with_attribute("hx-target", "main")
                                        .with_attribute("hx-vals", "{\"redirect_action\": \"List\"}")
                                        .with_attribute("class", "button-primary")
                                        .with_attribute("type", "submit")
                                        .with_content(Text::new("Add word and return to the list"))
                                )
                                .add_element(
                                    ClosableHtmlElement::new(Button)
                                        .with_attribute("hx-post", "/add-word")
                                        .with_attribute("hx-target", "main")
                                        .with_attribute("hx-vals", "{\"redirect_action\": \"Stay\"}")
                                        .with_attribute("class", "button-secondary")
                                        .with_attribute("type", "submit")
                                        .with_content(Text::new("Add word and stay on the page to add another"))
                                )
                        )
                )
        )
}