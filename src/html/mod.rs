use std::collections::HashMap;

#[derive(Clone)]
pub enum ClosableHtmlElementType {
    Html,
    Head,
    Body,
    Title,
    Script,
    Style,
    Div,
    Span,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    P,
    A,
    Img,
    Ul,
    Ol,
    Li,
    Table,
    Tr,
    Th,
    Td,
    Form,
    Button,
    Select,
    Option,
    Header,
    Main,
    Footer,
}

impl Into<String> for ClosableHtmlElementType {
    fn into(self) -> String {
        match self {
            ClosableHtmlElementType::Html => "html".to_string(),
            ClosableHtmlElementType::Head => "head".to_string(),
            ClosableHtmlElementType::Body => "body".to_string(),
            ClosableHtmlElementType::Title => "title".to_string(),
            ClosableHtmlElementType::Script => "script".to_string(),
            ClosableHtmlElementType::Style => "style".to_string(),
            ClosableHtmlElementType::Div => "div".to_string(),
            ClosableHtmlElementType::Span => "span".to_string(),
            ClosableHtmlElementType::H1 => "h1".to_string(),
            ClosableHtmlElementType::H2 => "h2".to_string(),
            ClosableHtmlElementType::H3 => "h3".to_string(),
            ClosableHtmlElementType::H4 => "h4".to_string(),
            ClosableHtmlElementType::H5 => "h5".to_string(),
            ClosableHtmlElementType::H6 => "h6".to_string(),
            ClosableHtmlElementType::P => "p".to_string(),
            ClosableHtmlElementType::A => "a".to_string(),
            ClosableHtmlElementType::Img => "img".to_string(),
            ClosableHtmlElementType::Ul => "ul".to_string(),
            ClosableHtmlElementType::Ol => "ol".to_string(),
            ClosableHtmlElementType::Li => "li".to_string(),
            ClosableHtmlElementType::Table => "table".to_string(),
            ClosableHtmlElementType::Tr => "tr".to_string(),
            ClosableHtmlElementType::Th => "th".to_string(),
            ClosableHtmlElementType::Td => "td".to_string(),
            ClosableHtmlElementType::Form => "form".to_string(),
            ClosableHtmlElementType::Button => "button".to_string(),
            ClosableHtmlElementType::Select => "select".to_string(),
            ClosableHtmlElementType::Option => "option".to_string(),
            ClosableHtmlElementType::Header => "header".to_string(),
            ClosableHtmlElementType::Main => "main".to_string(),
            ClosableHtmlElementType::Footer => "footer".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct ClosableHtmlElement {
    element_type: ClosableHtmlElementType,
    content: Option<String>,
    attributes: HashMap<String, String>,
}

impl ClosableHtmlElement {
    pub fn new(element_type: ClosableHtmlElementType) -> Self {
        Self {
            element_type,
            content: None,
            attributes: HashMap::new(),
        }
    }

    pub fn with_content(mut self, content: impl RenderableHtmlElement) -> Self {
        self.content = Some(content.render());
        self
    }

    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }
}

impl RenderableHtmlElement for ClosableHtmlElement {
    fn render(&self) -> String {
        let mut result = String::new();

        let element_string: String = self.element_type.clone().into();

        result.push_str(&format!("<{}", element_string));

        if !self.attributes.is_empty() {
            result.push_str(" ");
        }

        for (key, value) in &self.attributes {
            result.push_str(&format!("{}=\"{}\"", key, html_escape::encode_text(value)));
        }

        result.push_str(">");

        result.push_str(&self.content.clone().unwrap_or_default());

        result.push_str(&format!("</{}>", element_string));

        result
    }
}

#[derive(Clone)]
pub enum NonClosableHtmlElementType {
    Doctype,
    Br,
    Hr,
    Input,
    Meta,
    Link,
}

impl Into<String> for NonClosableHtmlElementType {
    fn into(self) -> String {
        match self {
            NonClosableHtmlElementType::Doctype => "!DOCTYPE html".to_string(),
            NonClosableHtmlElementType::Br => "br".to_string(),
            NonClosableHtmlElementType::Hr => "hr".to_string(),
            NonClosableHtmlElementType::Input => "input".to_string(),
            NonClosableHtmlElementType::Meta => "meta".to_string(),
            NonClosableHtmlElementType::Link => "link".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct NonClosableHtmlElement {
    element_type: NonClosableHtmlElementType,
    attributes: HashMap<String, String>,
}

impl NonClosableHtmlElement {
    pub fn new(element_type: NonClosableHtmlElementType) -> Self {
        Self {
            element_type,
            attributes: HashMap::new(),
        }
    }

    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());

        self
    }
}

pub trait RenderableHtmlElement {
    fn render(&self) -> String;
}

impl RenderableHtmlElement for NonClosableHtmlElement {
    fn render(&self) -> String {
        let element_type_stringified: String = self.element_type.clone().into();

        let mut element = format!("<{}", element_type_stringified);

        if !self.attributes.is_empty() {
            element.push_str(" ");
        }

        let attributes_stringified: String = self.attributes.iter().map(|(key, value)| {
            format!("{}=\"{}\"", key, html_escape::encode_text(value))
        }).collect::<Vec<String>>().join(" ");

        element.push_str(&attributes_stringified);

        element.push_str(">");

        element
    }
}

pub struct MultipleHtmlElements<'a> {
    elements: Vec<Box<dyn RenderableHtmlElement + 'a>>,
}

impl<'a> MultipleHtmlElements<'a> {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, element: impl RenderableHtmlElement + 'a) -> &mut Self {
        self.elements.push(Box::new(element));

        self
    }
}

impl<'a> RenderableHtmlElement for MultipleHtmlElements<'a> {
    fn render(&self) -> String {
        let mut elements = String::new();

        for element in &self.elements {
            elements.push_str(&element.render());
        }

        elements
    }
}

impl<'a> RenderableHtmlElement for &mut MultipleHtmlElements<'a> {
    fn render(&self) -> String {
        let mut elements = String::new();

        for element in &self.elements {
            elements.push_str(&element.render());
        }

        elements
    }
}

pub struct Text(pub String);

impl Text {
    pub fn new(text: impl Into<String>) -> Self {
        Self(text.into())
    }
}

impl RenderableHtmlElement for Text {
    fn render(&self) -> String {
        html_escape::encode_text(&self.0).to_string()
    }
}