use html::HtmlNode;
use textnode::{TextNode, TextType};

mod html;
mod textnode;

pub fn main() {
    let textnode = TextNode::new("this is a text node".to_string(), TextType::Bold, None);
    textnode.to_html();
}
