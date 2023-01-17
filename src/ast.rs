#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<'source> {
    tag: &'source str,
    attributes: Vec<Attribute<'source>>,
    children: Vec<Node<'source>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute<'source> {
    name: &'source str,
    value: Literal<'source>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal<'source> {
    Number(u32),
    String(&'source str),
    List(Vec<Literal<'source>>),
    Range { start: u32, end: u32 },
}
