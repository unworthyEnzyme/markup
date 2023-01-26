use crate::ast::Node;

pub trait Transformer {
    type Item;
    fn transform(&mut self) -> Self::Item;
}

#[derive(Debug, Clone)]
pub struct HtmlTransformer<'source> {
    ast: &'source Vec<Node<'source>>,
}

impl<'source> HtmlTransformer<'source> {
    pub fn new(ast: &'source Vec<Node<'source>>) -> Self {
        Self { ast }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Html<'source>(&'source str);

impl<'source> Transformer for HtmlTransformer<'source> {
    type Item = Html<'source>;
    fn transform(&mut self) -> Self::Item {
        let first_node = self.ast.get(0);
        match first_node {
            Some(Node::String(s)) => Html(s),
            _ => todo!(),
        }
    }
}
