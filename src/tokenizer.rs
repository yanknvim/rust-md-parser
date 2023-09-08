#[derive(Debug, Clone)]
pub enum Token {
    Text(String),
    Bold,
    Italic,
    Header(u32),
    Blockquote,
    List,
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
                tokens.push(Token::Header(depth));
            },
            '>' => {
                while let Some(' ') = chars.peek() {
                    chars.next();
                }
                tokens.push(Token::Blockquote);
            },
            '*' => {
                let mut depth = 1;
                while let Some('*') = chars.peek() {
                    depth += 1;
                    chars.next();
                }
                if !buffer.is_empty() {
                    tokens.push(Token::Text(buffer.clone()));
                    buffer.clear();
                }
                while let Some(' ') = chars.peek() {
                    chars.next();
                }
                match depth {
                    1 => tokens.push(Token::Italic),
                    2 => tokens.push(Token::Bold),
                    3 => {
                        tokens.push(Token::Italic);
                        tokens.push(Token::Bold);
                    },
                    _ => {}
                }
            },
            '-' => {
                while let Some(' ') = chars.peek() {
                    chars.next();
                }
                tokens.push(Token::List);
            }
            _ => buffer.push(c),
        }
    }
    if !buffer.is_empty() {
        tokens.push(Token::Text(buffer.clone()));
        buffer.clear();
    }
    tokens
}
