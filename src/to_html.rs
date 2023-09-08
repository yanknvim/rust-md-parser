use crate::parser::Node;

pub fn to_html(asts: Vec<Vec<Node>>) -> String {
    let mut html = String::new();
    for ast in asts {
        html.push_str(&to_html_line(ast));
    }
    html
}

pub fn to_html_line(ast: Vec<Node>) -> String {
    let mut html = String::new();
    for node in ast {
        match node {
            Node::Text(text) => {
                html.push_str(&text);
            },
            Node::Bold(child) => {
                html.push_str(&format!("<strong>{}</strong>", to_html_line(child)));
                html.push_str("\n");
            },
            Node::Italic(child) => {
                html.push_str(&format!("<em>{}</em>", to_html_line(child)));
                html.push_str("\n");
            },
            Node::Header(depth, child) => {
                html.push_str(&format!("<h{}>{}</h{}>", depth, to_html_line(child), depth));
                html.push_str("\n");
            },
            Node::Blockquote(child) => {
                html.push_str(&format!("<blockquote>{}</blockquote>", to_html_line(child)));
                html.push_str("\n");
            },
            Node::List(child) => {
                html.push_str(&format!("<ul><li>{}</li></ul>", to_html_line(child)));
                html.push_str("\n");
            },
            _ => {},
        }
    }
    html
}
