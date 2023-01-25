use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tag<'source> {
    pub name: &'source str,
    pub attributes: Vec<Attribute<'source>>,
    pub children: Vec<Node<'source>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Node<'source> {
    Tag(Tag<'source>),
    String(&'source str),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attribute<'source> {
    pub(crate) name: &'source str,
    pub(crate) value: Literal<'source>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Literal<'source> {
    Number(u32),
    String(&'source str),
    List(Vec<Literal<'source>>),
    Range { start: u32, end: Option<u32> },
}
