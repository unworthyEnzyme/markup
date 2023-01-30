use std::fs;

use markup::{
    ast::Literal,
    parser::Parser,
    plugin::{HtmlTransformer, Transformer},
};

fn main() {
    let source = r#"
code-block(highlights: [1, 3..5], lang: "ts") {
    "
        function add(a: number, b: number) {
            return a + b;
        }
        const result = add(1, 2);
        console.log(result);
    "
}
    "#;
    let node = Parser::new().parse(source.as_bytes()).unwrap();
    let transformed = HtmlTransformer.transform(&node);
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="UTF-8" />
        <meta http-equiv="X-UA-Compatible" content="IE=edge" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>Document</title>
    </head>
    <body>
        {}
    </body>
</html>
        "#,
        transformed
    );
    fs::write("source.html", html).unwrap();
}
