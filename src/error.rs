use crate::lexer::{Annot, LexError, LexErrorKind, Loc, Token, TokenKind};
use crate::parser::ParseError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Error {
    Lexer(LexError),
    Parser(ParseError),
}

impl From<LexError> for Error {
    fn from(e: LexError) -> Self {
        Error::Lexer(e)
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::Parser(e)
    }
}

use std::fmt;
impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::TokenKind::*;
        match self {
            Number(n) => n.fmt(f),
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            Asterisk => write!(f, "*"),
            Slash => write!(f, "/"),
            LParen => write!(f, "("),
            RParen => write!(f, ")"),
        }
    }
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.0, self.1)
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::LexErrorKind::*;
        let loc = &self.loc;
        match self.value {
            InvalidChar(c) => write!(f, "{}: invalid char '{}'", loc, c),
            Eof => write!(f, "End of file"),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ParseError::*;
        match self {
            UnexpectedToken(tok) => write!(f, "{}: {} is not expected", tok.loc, tok.value),
            NotExpression(tok) => write!(
                f,
                "{}: '{}' is not a start of expression",
                tok.loc, tok.value
            ),
            NotOperator(tok) => write!(f, "{}: '{}' is not an operator", tok.loc, tok.value),
            UnclosedOpenParen(tok) => write!(f, "{}: '{}' is not closed", tok.loc, tok.value),
            RedundantExpression(tok) => write!(
                f,
                "{}: expression after '{}' is redundant",
                tok.loc, tok.value
            ),
            Eof => write!(f, "End of file"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parser error")
    }
}

use std::error::Error as StdError;
impl StdError for LexError {}
impl StdError for ParseError {}
impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        use self::Error::*;
        match self {
            Lexer(lex) => Some(lex),
            Parser(parse) => Some(parse),
        }
    }
}

fn print_annot(input: &str, loc: Loc) {
    eprintln!("{}", input);
    eprintln!("{}{}", " ".repeat(loc.0), "^".repeat(loc.1 - loc.0));
}

impl Error {
    pub fn show_diagnostic(&self, input: &str) {
        use self::Error::*;
        use self::ParseError as P;
        let (e, loc): (&StdError, Loc) = match self {
            Lexer(e) => (e, e.loc.clone()),
            Parser(e) => {
                let loc = match e {
                    P::UnexpectedToken(Token { loc, .. })
                    | P::NotExpression(Token { loc, .. })
                    | P::NotOperator(Token { loc, .. })
                    | P::UnclosedOpenParen(Token { loc, .. }) => loc.clone(),
                    P::RedundantExpression(Token { loc, .. }) => Loc(loc.0, input.len()),
                    P::Eof => Loc(input.len(), input.len() + 1),
                };
                (e, loc)
            }
        };
        eprintln!("{}", e);
        print_annot(input, loc);
    }
}

pub fn show_trace<E: StdError>(e: E) {
    eprintln!("{}", e);
    let mut source = e.source();
    while let Some(e) = source {
        eprintln!("caused by {}", e);
        source = e.source()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InterpreterErrorKind {
    DivisionByZero,
}

pub type InterpreterError = Annot<InterpreterErrorKind>;

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::InterpreterErrorKind::*;
        match self.value {
            DivisionByZero => write!(f, "division by zero"),
        }
    }
}

impl StdError for InterpreterError {
    fn description(&self) -> &str {
        use self::InterpreterErrorKind::*;
        match self.value {
            DivisionByZero => "the right hand expression of the division evaluates to zero",
        }
    }
}

impl InterpreterError {
    pub fn show_diagnostic(&self, input: &str) {
        // エラー情報を簡単に表示し
        eprintln!("{}", self);
        // エラー位置を指示する
        print_annot(input, self.loc.clone());
    }
}
