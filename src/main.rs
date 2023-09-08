mod tokenizer;
mod parser;
mod to_html;

fn main() {
    let md = "# h1\n\
    ## h2\n\
    *italic*\n\
    **bold**\n\
    > blockquote\n\
    - list1\n\
    - list2\n\
    text\n\
    # *test*".to_string();
    let tokens = tokenizer::tokenize(md);
    for token_line in tokens {
        let ast = parser::parse(token_line);
        println!("{}", to_html::to_html(ast));
    }
}
