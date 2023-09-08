use crate::tokenizer::{Token, TokenKind};

#[derive(Debug, Clone)]
pub enum NodeKind {
    Text,
    Bold,
    Italic,
    Header,
    Blockquote,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub str: String,
    pub depth: u32,
    pub child: Option<Box<Vec<Node>>>,
}

pub fn parse(tokens: Vec<Token>) -> Vec<Node> {
    let mut nodes = Vec::new();

    let mut tokens = tokens.into_iter();
    while let Some(token) = tokens.next() {
        match token.kind {
            TokenKind::Text => {
                nodes.push(Node {
                    kind: NodeKind::Text,
                    str: token.str,
                    depth: 0,
                    child: None,
                });
            },
            TokenKind::Italic => {
                nodes.push(Node {
                    kind: NodeKind::Italic,
                    str: token.str,
                    depth: 0,
                    child: Some(Box::new(vec![Node {
                        kind: NodeKind::Text,
                        str: tokens.next().unwrap().str,
                        depth: 0,
                        child: None,
                    }]))
                });
                tokens.next();
            },
            TokenKind::Bold => {
                nodes.push(Node {
                    kind: NodeKind::Bold,
                    str: token.str,
                    depth: 0,
                    child: Some(Box::new(vec![Node {
                        kind: NodeKind::Text,
                        str: tokens.next().unwrap().str,
                        depth: 0,
                        child: None,
                    }]))
                });
                tokens.next();
            },
            TokenKind::Header => {
                nodes.push(Node {
                    kind:NodeKind::Header,
                    str: token.str,
                    depth: token.depth,
                    child: Some(Box::new(parse(tokens.clone().collect()))),
                })
            },
            TokenKind::Blockquote => {
                nodes.push(Node {
                    kind:NodeKind::Blockquote,
                    str: token.str,
                    depth: token.depth,
                    child: Some(Box::new(parse(tokens.clone().collect()))),
                })
            }
            _ => {},
        }
    }
    nodes
}
