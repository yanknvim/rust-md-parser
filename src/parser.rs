use crate::tokenizer::Token;

#[derive(Debug, Clone)]
pub enum Node {
    Text(String),
    Bold(Vec<Node>),
    Italic(Vec<Node>),
    Header(u32, Vec<Node>),
    Blockquote(Vec<Node>),
    List(Vec<Node>),
}

pub fn parse(tokens: Vec<Vec<Token>>) -> Vec<Vec<Node>> {
    let mut nodes = Vec::new();
    for line in tokens {
        nodes.push(parse_line(line));
    }
    nodes
}

pub fn parse_line(tokens: Vec<Token>) -> Vec<Node> {
    let mut nodes = Vec::new();

    let mut tokens = tokens.into_iter();
    while let Some(token) = tokens.next() {
        match token {
            Token::Text(text)=> {
                nodes.push(Node::Text(text));
            },
            Token::Italic => {
                nodes.push(Node::Italic(
                    parse_line(vec![tokens.next().unwrap()])
                ));
                tokens.next();
            },
            Token::Bold => {
                nodes.push(Node::Bold(
                    parse_line(vec![tokens.next().unwrap()])
                ));

                tokens.next();
            },
            Token::Header(depth) => {
                nodes.push(Node::Header(
                    depth,
                    parse_line(tokens.clone().collect::<Vec<_>>())
                ));
                break;
            },
            Token::Blockquote => {
                nodes.push(Node::Blockquote(
                    parse_line(tokens.clone().collect::<Vec<_>>())
                ));
                break;
            },
            Token::List => {
                nodes.push(Node::List(
                    parse_line(tokens.clone().collect::<Vec<_>>())
                ));
                break;
            }
            _ => {},
        }
    }
    nodes
}
