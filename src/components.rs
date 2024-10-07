use rand::random;
use serde::Deserialize;
use crate::html::{ClosableHtmlElement, MultipleHtmlElements, NonClosableHtmlElement, RenderableHtmlElement, Text, UnsafeText};
use crate::html::ClosableHtmlElementType::{Body, Button, Div, Form, Head, Header, Html, Label, Main, Script, Span, Table, Tbody, Td, Th, Thead, Title, Tr, A, H1, P};
use crate::html::NonClosableHtmlElementType::{Doctype, Img, Input, Link, Meta};
use crate::models::Word;

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
                                        .with_attribute("hx-vals", "{\"redirect_action\": \"List\"}")
                                        .with_attribute("class", "button-primary")
                                        .with_attribute("type", "submit")
                                        .with_content(Text::new("Add word and return to the list"))
                                )
                                .add_element(
                                    ClosableHtmlElement::new(Button)
                                        .with_attribute("hx-post", "/add-word")
                                        .with_attribute("hx-vals", "{\"redirect_action\": \"Stay\"}")
                                        .with_attribute("class", "button-secondary")
                                        .with_attribute("type", "submit")
                                        .with_content(Text::new("Add word and stay on the page to add another"))
                                )
                        )
                )
        )
}

pub fn word_list_component(words: Vec<Word>) -> impl RenderableHtmlElement {
    let words_content = if words.len() == 0 {
        ClosableHtmlElement::new(P)
            .with_attribute("class", "no-words-message")
            .with_content(Text::new("You have no words yet. Add them by clicking the button above."))
    } else {
        let mut tbody_elements = MultipleHtmlElements::new();

        for word in words {
            tbody_elements = tbody_elements.add_element(
                ClosableHtmlElement::new(Tr)
                    .with_content(
                        MultipleHtmlElements::new()
                            .add_element(
                                ClosableHtmlElement::new(Td)
                                    .with_attribute("class", "data-cell")
                                    .with_content(Text::new(word.word))
                            )
                            .add_element(
                                ClosableHtmlElement::new(Td)
                                    .with_attribute("class", "data-cell")
                                    .with_content(Text::new(word.translation))
                            )
                            .add_element(
                                ClosableHtmlElement::new(Td)
                                    .with_attribute("class", "data-cell")
                                    .with_content(
                                        ClosableHtmlElement::new(Button)
                                            .with_attribute("hx-confirm", "Are you sure you wish to delete this word from your list?")
                                            .with_attribute("hx-post", "/delete-word")
                                            .with_attribute("hx-vals", format!("{{\"id\": {}}}", word.id))
                                            .with_attribute("class", "clear-button")
                                            .with_content(UnsafeText::new("<svg xmlns=\"http://www.w3.org/2000/svg\" fill=\"none\" class=\"white-trash-icon\" viewBox=\"0 0 24 24\" stroke-width=\"1.5\">\r\n    <path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0\" />\r\n</svg>"))
                                    )
                            )
                    )
            );
        }

        ClosableHtmlElement::new(Table)
            .with_content(
                MultipleHtmlElements::new()
                    .add_element(
                        ClosableHtmlElement::new(Thead)
                            .with_content(
                                MultipleHtmlElements::new()
                                    .add_element(
                                        ClosableHtmlElement::new(Th)
                                            .with_content(Text::new("Word"))
                                    )
                                    .add_element(
                                        ClosableHtmlElement::new(Th)
                                            .with_content(Text::new("Translation"))
                                    )
                                    .add_element(
                                        ClosableHtmlElement::new(Th)
                                    )
                            )
                    )
                    .add_element(
                        ClosableHtmlElement::new(Tbody)
                            .with_content(tbody_elements)
                    )
            )
    };


    MultipleHtmlElements::new()
        .add_element(
            ClosableHtmlElement::new(Div)
                .with_attribute("class", "w-full flex flex-row gap-20")
                .with_content(MultipleHtmlElements::new()
                    .add_element(ClosableHtmlElement::new(A)
                        .with_attribute("href", "/add-word")
                        .with_attribute("hx-boost", "true")
                        .with_attribute("hx-target", "main")
                        .with_attribute("class", "button-secondary-big")
                        .with_content(
                            MultipleHtmlElements::new()
                                .add_element(UnsafeText::new("<svg xmlns=\"http://www.w3.org/2000/svg\" fill=\"none\" viewBox=\"0 0 24 24\" stroke-width=\"1.5\" stroke=\"currentColor\" class=\"button-icon\">\r\n  <path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"M12 9v6m3-3H9m12 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z\" />\r\n</svg>"))
                                .add_element(Text::new("Add word"))
                        ))
                    .add_element(
                        ClosableHtmlElement::new(A)
                            .with_attribute("href", "/teach")
                            .with_attribute("hx-boost", "true")
                            .with_attribute("hx-target", "main")
                            .with_attribute("class", "button-primary-big")
                            .with_content(
                                MultipleHtmlElements::new()
                                    .add_element(UnsafeText::new("<svg xmlns=\"http://www.w3.org/2000/svg\" fill=\"none\" viewBox=\"0 0 24 24\" stroke-width=\"1.5\" stroke=\"currentColor\" class=\"button-icon\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"M4.26 10.147a60.438 60.438 0 0 0-.491 6.347A48.62 48.62 0 0 1 12 20.904a48.62 48.62 0 0 1 8.232-4.41 60.46 60.46 0 0 0-.491-6.347m-15.482 0a50.636 50.636 0 0 0-2.658-.813A59.906 59.906 0 0 1 12 3.493a59.903 59.903 0 0 1 10.399 5.84c-.896.248-1.783.52-2.658.814m-15.482 0A50.717 50.717 0 0 1 12 13.489a50.702 50.702 0 0 1 7.74-3.342M6.75 15a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5Zm0 0v-3.675A55.378 55.378 0 0 1 12 8.443m-7.007 11.55A5.981 5.981 0 0 0 6.75 15.75v-1.5\" /></svg>"))
                                    .add_element(Text::new("Learn"))
                            )
                    )
                )
        )
        .add_element(
            words_content
        )
}

pub fn learn_word(translation: impl Into<String>, sentence: impl Into<String>) -> impl RenderableHtmlElement {
    ClosableHtmlElement::new(Div)
        .with_content(
            MultipleHtmlElements::new()
                .add_element(
                    ClosableHtmlElement::new(P)
                        .with_attribute("class", "learn-sentence")
                        .with_content(
                            MultipleHtmlElements::new()
                                .add_element(Text::new("Translation: "))
                                .add_element(ClosableHtmlElement::new(Span)
                                    .with_content(Text::new(translation)))
                        )
                )
                .add_element(
                    ClosableHtmlElement::new(P)
                        .with_attribute("class", "learn-sentence")
                        .with_content(
                            MultipleHtmlElements::new()
                                .add_element(Text::new("Sentence: "))
                                .add_element(ClosableHtmlElement::new(Span)
                                    .with_content(Text::new(sentence)))
                        )
                )
                .add_element(
                    NonClosableHtmlElement::new(Input)
                        .with_attribute("class", "learn-input")
                        .with_attribute("placeholder", "Write missing part here")
                        .with_attribute("autofocus", "")
                )
                .add_element(
                    ClosableHtmlElement::new(Div)
                        .with_attribute("class", "learn-actions")
                        .with_content(ClosableHtmlElement::new(Button)
                            .with_attribute("class", "button-primary-big max-300-px-width")
                            .with_content(Text::new("Check"))
                        )
                )
        )
}