mod html;
mod textnode;

pub fn main() {}

#[cfg(test)]
mod tests {
    use crate::{
        html::Html,
        textnode::{TextNode, TextType},
    };

    #[test]
    fn is_para() {
        let textnode = TextNode::new("this is a text node".to_string(), TextType::Bold, None);
        let html = textnode.to_html_node().to_string();
        assert_eq!(html, "<h>this is a text node</h>".to_string());
    }
}
