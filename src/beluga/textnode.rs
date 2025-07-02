use crate::beluga::html;
use html::{Html, HtmlNode, HtmlNodeType};

pub enum TextType {
    Plain,
    Bold,
    Italic,
    Code,
    Link,
    Img,
}

pub struct TextNode {
    pub text: String,
    pub text_type: TextType,
    pub link: Option<String>,
}

impl TextNode {
    pub fn new(text: String, typ: TextType, link: Option<String>) -> Self {
        Self {
            text,
            text_type: typ,
            link,
        }
    }
}

impl Html for TextNode {
    fn to_html_node(&self) -> HtmlNode {
        let (typ, link) = match self.text_type {
            TextType::Plain => (HtmlNodeType::Para, None),
            TextType::Bold => (HtmlNodeType::H1, None),
            TextType::Italic => (HtmlNodeType::Para, None),
            TextType::Code => (HtmlNodeType::Para, None),
            TextType::Link => (HtmlNodeType::Ahref, self.link.clone()),
            TextType::Img => (HtmlNodeType::Img, self.link.clone()),
        };

        HtmlNode::new(typ, &self.text, link)
    }
}
