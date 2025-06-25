use textnode::{TextNode, TextType};

use crate::html::Html;

mod html;
mod textnode;

pub fn main() {
    let textnode = TextNode::new("this is a text node".to_string(), TextType::Bold, None);
    let html = textnode.to_html_node().to_string();
    println!("{html}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_para() {
        let textnode = TextNode::new("this is a text node".to_string(), TextType::Bold, None);
        let html = textnode.to_html_node().to_string();
        assert_eq!(html, "<p> this is a text node </p>".to_string());
    }
}
