#[derive(Debug, Clone)]
pub enum TokenKind {
    Text,
    Bold,
    Italic,
    Header,
    Blockquote,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub str: String,
    pub depth: u32,
}

pub fn tokenize(s: String) -> Vec<Vec<Token>> {
    let mut tokens = Vec::new();
    for line in s.lines() {
        tokens.push(tokenize_line(line.to_string()));
    }
    tokens
}

pub fn tokenize_line(line: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();

    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '#' => {
                let mut depth = 1;
                while let Some('#') = chars.peek() {
                    depth += 1;
                    chars.next();
                }
                while let Some(' ') = chars.peek() {
                    chars.next();
                }
                tokens.push(Token {
                    kind: TokenKind::Header,
                    str: "#".to_string(),
                    depth: depth,
                })
            },
            '>' => {
                while let Some(' ') = chars.peek() {
                    chars.next();
                }
                tokens.push(Token {
                    kind: TokenKind::Blockquote,
                    str: ">".to_string(),
                    depth: 0,
                })
            },
            '*' => {
                let mut depth = 1;
                while let Some('*') = chars.peek() {
                    depth += 1;
                    chars.next();
                }
                if !buffer.is_empty() {
                    tokens.push(Token {
                        kind: TokenKind::Text,
                        str: buffer.clone(),
                        depth: 0,
                    });
                    buffer.clear();
                }
                while let Some(' ') = chars.peek() {
                    chars.next();
                }
                if depth == 1 {
                    tokens.push(Token {
                        kind: TokenKind::Italic,
                        str: "*".to_string(),
                        depth: depth,
                    })
                } else if depth == 2 {
                    tokens.push(Token {
                        kind: TokenKind::Bold,
                        str: "**".to_string(),
                        depth: depth
                    })
                }
            },
            _ => buffer.push(c),
        }
    }
    if !buffer.is_empty() {
        tokens.push(Token {
            kind: TokenKind::Text,
            str: buffer.clone(),
            depth: 0,
        });
        buffer.clear();
    }
    tokens
}
