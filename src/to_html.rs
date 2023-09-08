use crate::parser::{Node, NodeKind};

pub fn to_html(ast: Vec<Node>) -> String {
    let mut html = String::new();
    for node in ast {
        match node.kind {
            NodeKind::Text => {
                html.push_str(&node.str);
            },
            NodeKind::Bold => {
                html.push_str(&format!("<strong>{}</strong>", )
            }
        }
    }
    html
}
