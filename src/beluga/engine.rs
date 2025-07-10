use std::{
    fs::File,
    io::{BufRead, BufReader},
};

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
            HtmlNodeType::H1 => format!("<h1>{}</h1>\n", self.text),
            HtmlNodeType::H2 => format!("<h2>{}</h2>\n", self.text),
            HtmlNodeType::H3 => format!("<h3>{}</h3>\n", self.text),
            HtmlNodeType::Para => format!("<p>{}</p>\n", self.text),
            HtmlNodeType::Ahref => {
                if let Some(link) = &self.link {
                    format!(r#"<a href=\"{}\">{}</a>\n"#, link, self.text)
                } else {
                    format!("{}\n", self.text.clone())
                }
            }
            HtmlNodeType::Img => {
                if let Some(link) = &self.link {
                    format!(r#"<img src=\"{}\" alt=\"{}\"/>\n"#, link, self.text)
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

pub fn process_file_to_vec_of_nodes(file: File) -> Vec<HtmlNode> {
    let reader = BufReader::new(file);
    let mut nodes = Vec::new();
    let mut in_code_block = false;
    let mut code_buffer = String::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        if line.trim() == "```" {
            if in_code_block {
                // Close code block
                nodes.push(HtmlNode::new(
                    HtmlNodeType::Para,
                    format!("<pre><code>{}</code></pre>", code_buffer.trim()),
                    None,
                ));
                code_buffer.clear();
                in_code_block = false;
            } else {
                in_code_block = true;
            }
            continue;
        }

        if in_code_block {
            code_buffer.push_str(&line);
            code_buffer.push('\n');
            continue;
        }

        if line.starts_with("### ") {
            nodes.push(HtmlNode::new(
                HtmlNodeType::H3,
                line.trim_start_matches("### "),
                None,
            ));
        } else if line.starts_with("## ") {
            nodes.push(HtmlNode::new(
                HtmlNodeType::H2,
                line.trim_start_matches("## "),
                None,
            ));
        } else if line.starts_with("# ") {
            nodes.push(HtmlNode::new(
                HtmlNodeType::H1,
                line.trim_start_matches("# "),
                None,
            ));
        } else if line.starts_with("![") {
            // ![alt](url)
            if let Some((alt, rest)) = line.strip_prefix("![").and_then(|s| s.split_once("](")) {
                if let Some(link) = rest.strip_suffix(")") {
                    nodes.push(HtmlNode::new(
                        HtmlNodeType::Img,
                        alt.to_string(),
                        Some(link.to_string()),
                    ));
                }
            }
        } else if line.starts_with("[") {
            // [text](url)
            if let Some((text, rest)) = line.strip_prefix("[").and_then(|s| s.split_once("](")) {
                if let Some(link) = rest.strip_suffix(")") {
                    nodes.push(HtmlNode::new(
                        HtmlNodeType::Ahref,
                        text.to_string(),
                        Some(link.to_string()),
                    ));
                }
            }
        } else if !line.trim().is_empty() {
            nodes.push(HtmlNode::new(HtmlNodeType::Para, line, None));
        }
    }

    nodes
}

fn nodes_to_html_file(nodes: Vec<HtmlNode>) -> String {
    nodes
        .iter()
        .map(|node| node.to_string())
        .collect::<String>()
}

#[cfg(test)]
mod nodes_to_html_tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_process_file_to_vec_of_nodes() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            "# Heading 1\n\
             ## Heading 2\n\
             ### Heading 3\n\
             This is a paragraph.\n\
             [Google](https://google.com)\n\
             ![Alt](https://img.png)\n\
             ```\nfn main() {{}}\n```\n"
        )
        .unwrap();

        let nodes = process_file_to_vec_of_nodes(file.reopen().unwrap());
        assert_eq!(nodes.len(), 7);

        assert!(matches!(nodes[0].typ, HtmlNodeType::H1));
        assert_eq!(nodes[0].text, "Heading 1");

        assert!(matches!(nodes[1].typ, HtmlNodeType::H2));
        assert_eq!(nodes[1].text, "Heading 2");

        assert!(matches!(nodes[2].typ, HtmlNodeType::H3));
        assert_eq!(nodes[2].text, "Heading 3");

        assert!(matches!(nodes[3].typ, HtmlNodeType::Para));
        assert_eq!(nodes[3].text, "This is a paragraph.");

        assert!(matches!(nodes[4].typ, HtmlNodeType::Ahref));
        assert_eq!(nodes[4].text, "Google");
        assert_eq!(nodes[4].link.as_deref(), Some("https://google.com"));

        assert!(matches!(nodes[5].typ, HtmlNodeType::Img));
        assert_eq!(nodes[5].text, "Alt");
        assert_eq!(nodes[5].link.as_deref(), Some("https://img.png"));

        assert!(nodes[6].text.contains("fn main()"));
        assert!(nodes[6].text.contains("<pre><code>"));
    }

    #[test]
    fn test_nodes_to_html_file() {
        let nodes = vec![
            HtmlNode::new(HtmlNodeType::H1, "Title", None),
            HtmlNode::new(HtmlNodeType::Para, "Paragraph here.", None),
            HtmlNode::new(
                HtmlNodeType::Ahref,
                "Link",
                Some("https://test.com".to_string()),
            ),
        ];

        let html = nodes_to_html_file(nodes);
        assert!(html.contains("<h1>Title</h1>"));
        assert!(html.contains("<p>Paragraph here.</p>"));
        assert!(html.contains(r#"<a href="https://test.com">Link</a>"#));
    }
}
