pub enum HtmlNodeType {
    Para,
    H1,
    H2,
    H3,
    H4,
    Br,
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

pub trait Html {
    fn to_html_node(&self) -> HtmlNode;
}
