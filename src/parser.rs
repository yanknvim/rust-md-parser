use crate::tokenizer::Token;

#[derive(Debug, Clone)]
pub enum Node {
    Text(String),
    Bold(Vec<Node>),
    Italic(Vec<Node>),
    Header(u32, Vec<Node>),
    Blockquote(Vec<Node>),
}

pub fn parse(tokens: Vec<Token>) -> Vec<Node> {
    let mut nodes = Vec::new();

    let mut tokens = tokens.into_iter();
    while let Some(token) = tokens.next() {
        match token {
            Token::Text(text)=> {
                nodes.push(Node::Text(text));
            },
            Token::Italic => {
                nodes.push(Node::Italic(
                    parse(vec![tokens.next().unwrap()])
                ));
                tokens.next();
            },
            Token::Bold => {
                nodes.push(Node::Bold(
                    parse(vec![tokens.next().unwrap()])
                ));

                tokens.next();
            },
            Token::Header(depth) => {
                nodes.push(Node::Header(
                    depth,
                    parse(tokens.clone().collect::<Vec<_>>())
                ));
                break;
            },
            Token::Blockquote => {
                nodes.push(Node::Blockquote(
                    parse(tokens.clone().collect::<Vec<_>>())
                ));
                break;
            }
            _ => {},
        }
    }
    nodes
}
