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
    let ast = parser::parse(tokens);
    println!("{}", to_html::to_html(ast));
}
