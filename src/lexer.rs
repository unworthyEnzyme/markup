use std::fmt::Display;

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
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::EOF => write!(f, "\0"),
        }
    }
}

#[derive(Debug)]
pub struct Lexer<'source> {
    source: &'source [u8],
    tokens: Vec<Token<'source>>,
    current: usize,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source [u8]) -> Self {
        Self {
            source,
            tokens: vec![],
            current: 0,
        }
    }

    fn scan_token(&mut self, c: char) -> Result<Token, ()> {
        match c {
            '(' => Ok(Token::LeftParen),
            ')' => Ok(Token::RightParen),
            '{' => Ok(Token::LeftBrace),
            '}' => Ok(Token::RightBrace),
            '[' => Ok(Token::LeftBracket),
            ']' => Ok(Token::RightBracket),
            ':' => Ok(Token::Colon),
            '"' => Ok(self.string()?),
            _ if c.is_alphabetic() => Ok(self.identifier()?),
            _ if c.is_numeric() => Ok(self.number()?),
            _ => Err(()),
        }
    }

    fn string(&mut self) -> Result<Token, ()> {
        self.advance();
        let start = self.current;
        while self.peek() != '"' && !self.is_at_end() {
            self.advance();
        }

        if self.is_at_end() {
            return Err(());
        }

        let value = &self.source[start..self.current];
        Ok(Token::String(
            //There should be a better way
            std::str::from_utf8(value).expect("strings should be valid utf-8"),
        ))
    }

    fn identifier(&mut self) -> Result<Token, ()> {
        let start = self.current;
        while is_alphabetic(self.peek()) {
            self.advance();
        }
        let lexeme = &self.source[start..self.current];
        Ok(Token::Identifier(
            std::str::from_utf8(lexeme).expect("should be valid utf-8"),
        ))
    }

    fn number(&mut self) -> Result<Token, ()> {
        let start = self.current;
        while char::is_numeric(self.peek()) {
            self.advance();
        }
        let lexeme = std::str::from_utf8(&self.source[start..self.current])
            .expect("This should be a valid utf-8");
        let value = lexeme.parse::<u32>().unwrap();
        Ok(Token::Number(value))
    }

    fn peek_next(&self) -> char {
        if self.current + 1 != self.source.len() {
            return self.source[self.current + 1].into();
        }
        '\0'
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
}

fn is_alphabetic(c: char) -> bool {
    c.is_alphabetic() || c == '_' || c == '-'
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Token};

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
}
