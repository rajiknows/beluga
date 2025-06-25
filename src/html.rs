pub enum HtmlNodeType {
    Para,
    H,
    Ahref,
}

pub struct HtmlNode {
    typ: HtmlNodeType,
    text: String,
    link: Option<String>,
    img_link: Option<String>,
}

impl HtmlNode {
    pub fn new(
        typ: HtmlNodeType,
        text: &String,
        link: &Option<String>,
        img_link: &Option<String>,
    ) -> Self {
        Self {
            typ,
            text: text.to_owned(),
            link: link.to_owned(),
            img_link: img_link.to_owned(),
        }
    }
}

impl ToString for HtmlNode {
    fn to_string(&self) -> String {
        let text = &self.text;
        match self.typ {
            HtmlNodeType::H => format!("<h> {text} </h>"),
            HtmlNodeType::Para => format!("<p> {text} </p>"),
            HtmlNodeType::Ahref => {
                let link = &self.link.clone().unwrap();
                format!(r#"<a href="{}">{}</a>"#, link, text)
            }
        }
    }
}

pub trait Html {
    fn to_html_node(&self) -> HtmlNode;
}
