mod tokenizer;
mod parser;
mod to_html;

fn main() {
    let md = "# h1\n## h2\n*italic*\n**bold**\n# *into a chaos*".to_string();
    let tokens = tokenizer::tokenize(md);
    println!("{:?}", tokens);
    for token_line in tokens {
        let ast = parser::parse(token_line);
        println!("{:?}", ast);
    }
}
