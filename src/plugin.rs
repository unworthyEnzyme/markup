use crate::ast::Node;

pub trait Transformer<'a> {
    type Item;
    fn transform(&mut self, ast: &'a Vec<Node<'a>>) -> Self::Item;
}

#[derive(Debug, Clone)]
pub struct HtmlTransformer;

impl<'source> Transformer<'source> for HtmlTransformer {
    type Item = String;
    fn transform(&mut self, ast: &'source Vec<Node<'source>>) -> Self::Item {
        let mut s = String::new();
        for node in ast {
            s.push_str(&transform_node(&node));
        }
        s
    }
}

fn transform_node(node: &Node) -> String {
    match node {
        Node::String(s) => String::from(*s),
        Node::Tag(t) => {
            let mut s = String::new();
            s.push_str(&format!("<{}>", t.name));
            let mut transformer = HtmlTransformer;
            let inner = transformer.transform(&t.children);
            s.push_str(&format!("{}</{}>", &inner, t.name));
            s
        }
    }
}
