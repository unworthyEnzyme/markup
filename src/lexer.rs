use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'a> {
    Identifier(&'a str),
    String(&'a str),
    Number(u32),
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    DoubleDot,
    Dot,
    Comma,
    Colon,
    EOF,
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Identifier(i) => write!(f, "{i}"),
            Token::String(s) => write!(f, "{s}"),
            Token::Number(n) => write!(f, "{n}"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBracket => write!(f, "["),
            Token::RightBracket => write!(f, "]"),
            Token::LeftBrace => write!(f, "`{{`"),
            Token::RightBrace => write!(f, "`}}`"),
            Token::DoubleDot => write!(f, ".."),
            Token::Dot => write!(f, "."),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::EOF => write!(f, "\0"),
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum LexingError {
    #[error("Unrecognized Character {} at position {}", .character, .position)]
    UnrecognizedCharacter { character: char, position: usize },
}

type LexingResult<'source> = Result<Token<'source>, LexingError>;

#[derive(Debug)]
pub struct Lexer<'source> {
    source: &'source [u8],
    current: usize,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source [u8]) -> Self {
        Self { source, current: 0 }
    }

    pub fn scan_tokens(&mut self) -> Vec<LexingResult<'source>> {
        let mut tokens = vec![];
        while !self.is_at_end() {
            let token = self.scan_token();
            tokens.push(token);
        }
        tokens.push(Ok(Token::EOF));
        tokens
    }

    fn scan_token(&mut self) -> LexingResult<'source> {
        let c = self.advance();
        match c {
            '(' => Ok(Token::LeftParen),
            ')' => Ok(Token::RightParen),
            '{' => Ok(Token::LeftBrace),
            '}' => Ok(Token::RightBrace),
            '[' => Ok(Token::LeftBracket),
            ']' => Ok(Token::RightBracket),
            ':' => Ok(Token::Colon),
            ',' => Ok(Token::Comma),
            '"' => Ok(self.string()?),
            '.' => {
                if self.match_char('.') {
                    Ok(Token::DoubleDot)
                } else {
                    Ok(Token::Dot)
                }
            }
            _ if c.is_alphabetic() => Ok(self.identifier()?),
            _ if c.is_numeric() => Ok(self.number()?),
            c @ _ => Err(LexingError::UnrecognizedCharacter {
                character: c,
                position: self.current,
            }),
        }
    }

    fn string(&mut self) -> LexingResult<'source> {
        let start = self.current;
        while self.peek() != '"' && !self.is_at_end() {
            self.advance();
        }

        if self.is_at_end() {
            return Err(LexingError::UnrecognizedCharacter {
                character: self.peek(),
                position: self.current,
            });
        }

        let value = &self.source[start..self.current];
        Ok(Token::String(
            //There should be a better way
            std::str::from_utf8(value).expect("strings should be valid utf-8"),
        ))
    }

    fn identifier(&mut self) -> LexingResult<'source> {
        let start = self.current;
        while is_alphabetic(self.peek()) {
            self.advance();
        }
        let lexeme = &self.source[start..self.current];
        Ok(Token::Identifier(
            std::str::from_utf8(lexeme).expect("should be valid utf-8"),
        ))
    }

    fn number(&mut self) -> LexingResult<'source> {
        let start = self.current - 1;
        while char::is_numeric(self.peek()) {
            self.advance();
        }
        let lexeme = std::str::from_utf8(&self.source[start..self.current])
            .expect("This should be a valid utf-8");
        let value = lexeme.parse::<u32>().unwrap();
        Ok(Token::Number(value))
    }

    fn peek(&self) -> char {
        if !self.is_at_end() {
            return self.source[self.current].into();
        }
        '\0'
    }

    fn advance(&mut self) -> char {
        if !self.is_at_end() {
            self.current += 1;
            return self.source[self.current - 1].into();
        }
        '\0'
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn match_char(&mut self, c: char) -> bool {
        if self.is_at_end() || self.source[self.current] as char != c {
            false
        } else {
            self.current += 1;
            true
        }
    }
}

fn is_alphabetic(c: char) -> bool {
    c.is_alphabetic() || c == '_' || c == '-'
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::{Lexer, Token};
    use pretty_assertions::assert_eq;

    #[test]
    fn number_literal() {
        let source = "123";
        let mut lexer = Lexer::new(source.as_bytes());
        let token = lexer.number();
        assert_eq!(token, Ok(Token::Number(123)))
    }

    #[test]
    fn string_literal() {
        let source = r#""this is a string literal""#;
        let mut lexer = Lexer::new(source.as_bytes());
        let token = lexer.string();
        assert_eq!(token, Ok(Token::String("this is a string literal")));
    }

    #[test]
    fn multiline_string_literal() {
        let source = r#""a multiline
string literal""#;
        let mut lexer = Lexer::new(source.as_bytes());
        let token = lexer.string();
        assert_eq!(token, Ok(Token::String("a multiline\nstring literal")))
    }

    #[test]
    fn identifier() {
        let source = "js-code_block";
        let mut lexer = Lexer::new(source.as_bytes());
        let token = lexer.identifier();
        assert_eq!(token, Ok(Token::Identifier("js-code_block")));
    }

    #[test]
    fn punctuation() {
        let source = "(){}[],..:";
        let mut lexer = Lexer::new(source.as_bytes());
        let tokens = lexer.scan_tokens();
        assert_eq!(
            tokens,
            vec![
                Ok(Token::LeftParen),
                Ok(Token::RightParen),
                Ok(Token::LeftBrace),
                Ok(Token::RightBrace),
                Ok(Token::LeftBracket),
                Ok(Token::RightBracket),
                Ok(Token::Comma),
                Ok(Token::DoubleDot),
                Ok(Token::Colon),
                Ok(Token::EOF)
            ]
        )
    }
}
