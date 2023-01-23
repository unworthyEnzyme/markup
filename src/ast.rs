#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag<'source> {
    pub name: &'source str,
    pub attributes: Vec<Attribute<'source>>,
    pub children: Vec<Node<'source>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node<'source> {
    Tag(Tag<'source>),
    String(&'source str),
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
