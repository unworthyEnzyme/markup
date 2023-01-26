use markup::{
    parser::Parser,
    plugin::{HtmlTransformer, Transformer},
};

fn main() {
    let source = r#""this is a string node""#;
    let node = Parser::new().parse(source.as_bytes()).unwrap();
    let mut transformer = HtmlTransformer::new(&node);
    let transformed = transformer.transform();
    println!("{transformed:?}");
}
