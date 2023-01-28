use crate::ast::Node;
use textwrap;

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
            if t.name == "code-block" {
                let mut s = String::new();
                s.push_str("<pre>");
                let mut transformer = HtmlTransformer;
                let inner = transformer.transform(&t.children);
                let inner = textwrap::dedent(&inner);
                s.push_str(&format!("<code>{}</code></pre>", inner));
                s
            } else {
                panic!("Only `code-block` is supported for now")
            }
        }
    }
}
