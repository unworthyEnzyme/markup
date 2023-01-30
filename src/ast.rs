use std::fmt::Display;

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

impl<'source> Display for Literal<'source> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::String(s) => write!(f, "{s}"),
            Self::Range { start, end } => {
                write!(
                    f,
                    "{start}..{}",
                    end.map_or(Self::String(""), |end| Self::Number(end))
                )
            }
            Self::List(ls) => {
                write!(f, "[")?;
                for l in 0..ls.len() - 1 {
                    write!(f, "{},", ls[l])?;
                }
                write!(f, "{}", ls.last().map_or(Self::String(""), |l| l.clone()))?;
                write!(f, "]")
            }
        }
    }
}
