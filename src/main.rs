use std::fs;

use markup::parser::Parser;

fn main() {
    let source = fs::read_to_string("source.txt").unwrap();
    let node = Parser::new().parse(source.as_bytes()).unwrap();
    let serialized = serde_json::to_string_pretty(&node).unwrap();
    fs::write("source.json", &serialized).unwrap();
}
