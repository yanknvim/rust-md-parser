use crate::parser::Node;

pub fn to_html(ast: Vec<Node>) -> String {
    let mut html = String::new();
    for node in ast {
        match node {
            Node::Text(text) => {
                html.push_str(&text);
            },
            Node::Bold(child) => {
                html.push_str(&format!("<strong>{}</strong>", to_html(child)));
            },
            Node::Italic(child) => {
                html.push_str(&format!("<em>{}</em>", to_html(child)));
            },
            Node::Header(depth, child) => {
                html.push_str(&format!("<h{}>{}</h{}>", depth, to_html(child), depth));
            },
            Node::Blockquote(child) => {
                html.push_str(&format!("<blockquote>{}</blockquote>", to_html(child)));
            },
            _ => {},
        }
    }
    html
}
