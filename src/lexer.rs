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
    #[error("Unclosed string literal at start: {} end: {}", .start, .end)]
    UnclosedStringLiteral { start: usize, end: usize },
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

    pub fn scan_tokens(&mut self) -> Result<Vec<Token<'source>>, LexingError> {
        let mut tokens = vec![];
        while !self.is_at_end() {
            self.skip_whitespace();
            let token = self.scan_token()?;
            tokens.push(token);
        }
        if tokens.last() != Some(&Token::EOF) {
            //if there is no whitespace after the last scanned token than we never call the scan token again so
            //we don't call the `scan_token` and get `Token::EOF`
            tokens.push(Token::EOF);
        }
        Ok(tokens)
    }

    fn scan_token(&mut self) -> LexingResult<'source> {
        let c = self.advance();
        match c {
            '(' => Ok(Token::LeftParen),
            ')' => Ok(Token::RightParen),
            '[' => Ok(Token::LeftBracket),
            ']' => Ok(Token::RightBracket),
            '{' => Ok(Token::LeftBrace),
            '}' => Ok(Token::RightBrace),
            ',' => Ok(Token::Comma),
            '.' => {
                if self.match_char('.') {
                    Ok(Token::DoubleDot)
                } else {
                    Ok(Token::Dot)
                }
            }
            ':' => Ok(Token::Colon),
            '\0' => Ok(Token::EOF),
            '"' => Ok(self.string()?),
            _ if c.is_numeric() => Ok(self.number()?),
            _ if c.is_alphabetic() => Ok(self.identifier()?),
            _ => Err(LexingError::UnrecognizedCharacter {
                character: c,
                position: self.current,
            }),
        }
    }

    fn string(&mut self) -> LexingResult<'source> {
        let start = self.current;
        while !self.is_at_end() && self.peek() != '"' {
            self.advance();
        }
        if self.is_at_end() {
            return Err(LexingError::UnclosedStringLiteral {
                start: start - 1,
                end: self.current,
            });
        }
        let inner = std::str::from_utf8(&self.source[start..self.current]).expect("should be utf8");
        //eat closing quote
        self.advance();
        Ok(Token::String(inner))
    }

    fn identifier(&mut self) -> LexingResult<'source> {
        let start = self.current - 1;
        while is_alphabetic(self.peek()) {
            self.advance();
        }
        let lexeme =
            std::str::from_utf8(&self.source[start..self.current]).expect("should be utf8");
        Ok(Token::Identifier(lexeme))
    }

    fn number(&mut self) -> LexingResult<'source> {
        let start = self.current - 1;
        while self.peek().is_numeric() {
            self.advance();
        }
        let lexeme =
            std::str::from_utf8(&self.source[start..self.current]).expect("Should be utf8");
        let num: u32 = lexeme
            .parse()
            .unwrap_or_else(|_| panic!("'{lexeme}' should be valid number"));
        Ok(Token::Number(num))
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current].into()
        }
    }

    fn advance(&mut self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.current += 1;
            self.source[self.current - 1].into()
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn match_char(&mut self, c: char) -> bool {
        if self.is_at_end() || self.peek() != c {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.peek().is_whitespace() {
            self.advance();
        }
    }
}

fn is_alphabetic(c: char) -> bool {
    c.is_alphabetic() || c == '_' || c == '-' || c.is_numeric()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::{Lexer, LexingError, Token};
    use pretty_assertions::assert_eq;

    #[test]
    fn number_literal() {
        let source = "123";
        let mut lexer = Lexer::new(source.as_bytes());
        let token = lexer.scan_tokens();
        assert_eq!(token, Ok(vec![Token::Number(123), Token::EOF]));
    }

    #[test]
    fn string_literal() {
        let source = r#""this is a string literal""#;
        let mut lexer = Lexer::new(source.as_bytes());
        let token = lexer.scan_tokens();
        assert_eq!(
            token,
            Ok(vec![Token::String("this is a string literal"), Token::EOF])
        );
    }

    #[test]
    fn unclosed_string_literal() {
        let source = r#""unclosed string"#;
        let mut lexer = Lexer::new(source.as_bytes());
        let tokens = lexer.scan_tokens();
        assert_eq!(
            tokens,
            Err(LexingError::UnclosedStringLiteral {
                start: 0,
                end: lexer.current
            })
        )
    }

    #[test]
    fn multiline_string_literal() {
        let source = r#""a multiline
string literal""#;
        let mut lexer = Lexer::new(source.as_bytes());
        let token = lexer.scan_tokens();
        assert_eq!(
            token,
            Ok(vec![
                Token::String("a multiline\nstring literal"),
                Token::EOF
            ])
        )
    }

    #[test]
    fn identifier() {
        let source = "js-code_block";
        let mut lexer = Lexer::new(source.as_bytes());
        let token = lexer.scan_tokens();
        assert_eq!(
            token,
            Ok(vec![Token::Identifier("js-code_block"), Token::EOF])
        );
    }

    #[test]
    fn punctuation() {
        let source = " ( ){}[ ] , ..: ";
        let mut lexer = Lexer::new(source.as_bytes());
        let tokens = lexer.scan_tokens();
        assert_eq!(
            tokens,
            Ok(vec![
                Token::LeftParen,
                Token::RightParen,
                Token::LeftBrace,
                Token::RightBrace,
                Token::LeftBracket,
                Token::RightBracket,
                Token::Comma,
                Token::DoubleDot,
                Token::Colon,
                Token::EOF
            ])
        )
    }
}
