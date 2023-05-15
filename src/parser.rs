use crate::{
    ast::{Attribute, Literal, Node, Tag},
    lexer::{Lexer, LexingError, Token},
};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum ParsingError<'a> {
    #[error("Unrecognized Character {} at position {}", .character, .position)]
    UnrecognizedCharacter { character: char, position: usize },
    #[error("Unclosed string literal at start: {} end: {}", .start, .end)]
    UnclosedStringLiteral { start: usize, end: usize },
    #[error("Unexpected token at {}. Expected an identifier or a string literal", .at)]
    UnexpectedToken { at: usize },
    #[error("Expected token {} and got {} at {}", .expected, .got, .at)]
    ExpectedToken {
        at: usize,
        expected: Token<'a>,
        got: Token<'a>,
    },
    #[error("Lexing error")]
    LexingError(#[from] LexingError),
}

#[derive(Debug, Clone)]
pub struct Parser<'source> {
    tokens: Vec<Token<'source>>,
    current: usize,
}

impl<'a> Default for Parser<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Self {
            tokens: vec![],
            current: 0,
        }
    }

    pub fn parse(&mut self, source: &'a [u8]) -> Result<Vec<Node<'a>>, ParsingError<'a>> {
        let mut lexer = Lexer::new(source);
        self.tokens = lexer.scan_tokens()?;
        let mut nodes: Vec<Node<'a>> = vec![];
        while !self.is_at_end() {
            nodes.push(self.node()?);
        }
        Ok(nodes)
    }

    fn node(&mut self) -> Result<Node<'a>, ParsingError<'a>> {
        match self.tokens[self.current] {
            Token::String(s) => {
                self.current += 1;
                Ok(Node::String(s))
            }
            Token::Identifier(_) => self.tag(),
            _ => Err(ParsingError::UnexpectedToken { at: self.current }),
        }
    }

    fn tag(&mut self) -> Result<Node<'a>, ParsingError<'a>> {
        let Token::Identifier(name) = self.tokens[self.current] else {
            return Err(ParsingError::UnexpectedToken { at: self.current });
        };
        self.current += 1;
        let mut node = Tag {
            name,
            attributes: vec![],
            children: vec![],
        };

        if let Token::LeftParen = self.tokens[self.current] {
            node.attributes = self.attributes()?;
        }

        self.consume(Token::LeftBrace)?;
        while self.tokens[self.current] != Token::RightBrace {
            node.children.push(self.node()?);
        }
        self.consume(Token::RightBrace)?;
        Ok(Node::Tag(node))
    }

    fn attributes(&mut self) -> Result<Vec<Attribute<'a>>, ParsingError<'a>> {
        self.consume(Token::LeftParen)?;
        let mut attrs: Vec<Attribute<'a>> = vec![];
        //This is similar to parsing lists except we call `self.attribute()` instead of `self.literal()`
        //Maybe i can extract the logic of surrounded and delimited grammars.
        if let Ok(attr) = self.attribute() {
            attrs.push(attr);
        }
        while self.tokens[self.current] == Token::Comma {
            self.current += 1;
            attrs.push(self.attribute()?);
        }

        self.consume(Token::RightParen)?;
        Ok(attrs)
    }

    fn attribute(&mut self) -> Result<Attribute<'a>, ParsingError<'a>> {
        let Token::Identifier(name) = self.tokens[self.current] else {
            return Err(ParsingError::UnexpectedToken { at: self.current });
        };
        self.current += 1;
        self.consume(Token::Colon)?;
        match self.tokens[self.current] {
            Token::Number(n) => {
                if self.peek_next() == Token::DoubleDot {
                    Ok(Attribute {
                        name,
                        value: self.range()?,
                    })
                } else {
                    self.current += 1;
                    Ok(Attribute {
                        name,
                        value: Literal::Number(n),
                    })
                }
            }
            Token::String(s) => {
                self.current += 1;
                Ok(Attribute {
                    name,
                    value: Literal::String(s),
                })
            }
            Token::LeftBracket => Ok(Attribute {
                name,
                value: self.list()?,
            }),
            _ => Err(ParsingError::UnexpectedToken { at: self.current }),
        }
    }

    fn list(&mut self) -> Result<Literal<'a>, ParsingError<'a>> {
        let mut items: Vec<Literal<'a>> = vec![];
        self.consume(Token::LeftBracket)?;
        //Because the non-terminal is `list ::= '[' (literal (',' literal))? ']'` like this
        //we first take the first literal and as long as we have a ',' we parse the others in
        //the while loop.
        if let Ok(l) = self.literal() {
            items.push(l);
        }
        while self.tokens[self.current] == Token::Comma {
            self.current += 1;
            items.push(self.literal()?);
        }

        self.consume(Token::RightBracket)?;
        Ok(Literal::List(items))
    }

    fn range(&mut self) -> Result<Literal<'a>, ParsingError<'a>> {
        let Token::Number(start) = self.tokens[self.current] else {
            return Err(ParsingError::UnexpectedToken { at: self.current });
        };
        self.current += 1;
        self.consume(Token::DoubleDot)?;
        if let Token::Number(end) = self.tokens[self.current] {
            self.current += 1;
            Ok(Literal::Range {
                start,
                end: Some(end),
            })
        } else {
            Ok(Literal::Range { start, end: None })
        }
    }

    fn literal(&mut self) -> Result<Literal<'a>, ParsingError<'a>> {
        match self.tokens[self.current] {
            Token::Number(n) => {
                if self.peek_next() == Token::DoubleDot {
                    Ok(self.range()?)
                } else {
                    self.current += 1;
                    Ok(Literal::Number(n))
                }
            }
            Token::String(s) => {
                self.current += 1;
                Ok(Literal::String(s))
            }
            Token::LeftBracket => Ok(self.list()?),
            _ => Err(ParsingError::UnexpectedToken { at: self.current }),
        }
    }

    fn is_at_end(&self) -> bool {
        self.tokens[self.current] == Token::EOF
    }
    fn consume(&mut self, token: Token<'a>) -> Result<(), ParsingError<'a>> {
        if self.tokens[self.current] == token {
            self.current += 1;
            Ok(())
        } else {
            Err(ParsingError::ExpectedToken {
                at: self.current,
                expected: token,
                got: self.tokens[self.current],
            })
        }
    }

    fn peek_next(&self) -> Token<'a> {
        self.tokens[self.current + 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Attribute, Literal, Node, Tag},
        lexer::Lexer,
    };
    use pretty_assertions::assert_eq;

    use super::Parser;

    fn init_parser(source: &str) -> Parser {
        let mut lexer = Lexer::new(source.as_bytes());
        let tokens = lexer.scan_tokens().unwrap();
        let mut parser = Parser::new();
        parser.tokens = tokens;
        parser
    }

    #[test]
    fn range() {
        let source = "1..10";
        let mut parser = init_parser(source);
        let ast = parser.range();
        assert_eq!(
            ast,
            Ok(Literal::Range {
                start: 1,
                end: Some(10)
            })
        )
    }

    #[test]
    fn open_range() {
        let source = "1..";
        let mut parser = init_parser(source);
        let ast = parser.range();
        assert_eq!(
            ast,
            Ok(Literal::Range {
                start: 1,
                end: None
            })
        )
    }

    #[test]
    fn list() {
        let source = r#"[1, 1..3, "string"]"#;
        let mut parser = init_parser(source);
        let ast = parser.list();
        assert_eq!(
            ast,
            Ok(Literal::List(vec![
                Literal::Number(1),
                Literal::Range {
                    start: 1,
                    end: Some(3)
                },
                Literal::String("string")
            ],))
        )
    }

    #[test]
    fn nested_list() {
        let source = "[1, 1..3, [1, 1..3]]";
        let mut parser = init_parser(source);
        let ast = parser.list();
        assert_eq!(
            ast,
            Ok(Literal::List(vec![
                Literal::Number(1),
                Literal::Range {
                    start: 1,
                    end: Some(3)
                },
                Literal::List(vec![
                    Literal::Number(1),
                    Literal::Range {
                        start: 1,
                        end: Some(3)
                    },
                ])
            ],))
        )
    }

    #[test]
    fn attribute() {
        let source = "name: [1, 1..3]";
        let mut parser = init_parser(source);
        let ast = parser.attribute();
        assert_eq!(
            ast,
            Ok(Attribute {
                name: "name",
                value: Literal::List(vec![
                    Literal::Number(1),
                    Literal::Range {
                        start: 1,
                        end: Some(3)
                    }
                ])
            })
        )
    }

    #[test]
    fn attributes() {
        let source = "(name: [1, 1..3], num: 123)";
        let mut parser = init_parser(source);
        let ast = parser.attributes();
        assert_eq!(
            ast,
            Ok(vec![
                Attribute {
                    name: "name",
                    value: Literal::List(vec![
                        Literal::Number(1),
                        Literal::Range {
                            start: 1,
                            end: Some(3)
                        }
                    ])
                },
                Attribute {
                    name: "num",
                    value: Literal::Number(123)
                }
            ])
        )
    }

    #[test]
    fn node() {
        let source = r#"
row(reversed: "true") {
    p { "first" }
    "second"
}"#;
        let mut parser = init_parser(source);
        let ast = parser.node();
        assert_eq!(
            ast,
            Ok(Node::Tag(Tag {
                name: "row",
                attributes: vec![Attribute {
                    name: "reversed",
                    value: Literal::String("true")
                }],
                children: vec![
                    Node::Tag(Tag {
                        name: "p",
                        attributes: vec![],
                        children: vec![Node::String("first")]
                    }),
                    Node::String("second")
                ]
            }))
        )
    }

    #[test]
    fn markup() {
        let source = r#"
p {"first"}
div {"second"}
"third"
"#;
        let mut parser = Parser::new();
        let ast = parser.parse(source.as_bytes()).unwrap();
        assert_eq!(
            ast,
            vec![
                Node::Tag(Tag {
                    name: "p",
                    attributes: vec![],
                    children: vec![Node::String("first")]
                }),
                Node::Tag(Tag {
                    name: "div",
                    attributes: vec![],
                    children: vec![Node::String("second")]
                }),
                Node::String("third")
            ]
        )
    }
}
