use markup::{
    parser::Parser,
    plugin::{HtmlTransformer, Transformer},
};

fn main() {
    let source = r#"
div {
    div {"item1"}
    div {"item2"}
}
    "#;
    let node = Parser::new().parse(source.as_bytes()).unwrap();
    let transformed = HtmlTransformer.transform(&node);
    println!("{}", transformed);
}
