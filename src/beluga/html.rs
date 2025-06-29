#[derive(Debug)]
pub enum HtmlNodeType {
    Para,
    H1,
    H2,
    H3,
    Ahref,
    Img,
}

#[derive(Debug)]
pub struct HtmlNode {
    pub typ: HtmlNodeType,
    pub text: String,
    pub link: Option<String>,
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
            HtmlNodeType::H1 => format!("<h1>{}</h1>", self.text),
            HtmlNodeType::H2 => format!("<h2>{}</h2>", self.text),
            HtmlNodeType::H3 => format!("<h3>{}</h3>", self.text),
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
