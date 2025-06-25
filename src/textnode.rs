use crate::html::{Html, HtmlNode, HtmlNodeType};

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
        let node = HtmlNode::new(HtmlNodeType::Para, &self.text, &self.link, &None);
        return node;
    }
}
