#[derive(Debug)]
pub enum HtmlNodeType {
    Para,
    H,
    Ahref,
    Img,
}

#[derive(Debug)]
pub struct HtmlNode {
    typ: HtmlNodeType,
    text: String,
    link: Option<String>,
}

impl HtmlNode {
    pub fn new<T: Into<String>>(typ: HtmlNodeType, text: T, link: Option<String>) -> Self {
        Self {
            typ,
            text: text.into(),
            link,
        }
    }
}

impl ToString for HtmlNode {
    fn to_string(&self) -> String {
        match self.typ {
            HtmlNodeType::H => format!("<h>{}</h>", self.text),
            HtmlNodeType::Para => format!("<p>{}</p>", self.text),
            HtmlNodeType::Ahref => {
                if let Some(link) = &self.link {
                    format!(r#"<a href="{}">{}</a>"#, link, self.text)
                } else {
                    self.text.clone()
                }
            }
            HtmlNodeType::Img => {
                if let Some(link) = &self.link {
                    format!(r#"<img src="{}" alt="{}"/>"#, link, self.text)
                } else {
                    String::new()
                }
            }
        }
    }
}

pub trait Html {
    fn to_html_node(&self) -> HtmlNode;
}
