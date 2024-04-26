use std::io::Result;
use std::iter::Peekable;
use std::vec::IntoIter;

use crate::error::new_error;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Exit,
    For,
    EndFor,
    While,
    EndWhile,
    In,
    Assign,
    EndLine,
    OpenParen,
    CloseParen,
    VarName(String),
    Int(String),
    Operator(String),
    Callable(String),
}

// get words
// get tokens

pub struct Lexer {
    chars: Peekable<IntoIter<u8>>,
}

impl Lexer {
    pub fn lex(code: String) -> Result<Vec<TokenKind>> {
        Self {
            chars: code.into_bytes().into_iter().peekable(),
        }
        .lex_code()
    }

    fn lex_code(&mut self) -> Result<Vec<TokenKind>> {
        let mut tokens: Vec<TokenKind> = Vec::new();
        loop {
            let byte = match self.chars.next() {
                Some(byte) => byte,
                None => break,
            };
            let token = match byte {
                b' ' => continue,
                b'=' => Ok(TokenKind::Assign),
                b';' => Ok(TokenKind::EndLine),
                b'(' => Ok(TokenKind::OpenParen),
                b')' => Ok(TokenKind::CloseParen),
                b'+' | b'-' | b'/' | b'*' => Ok(TokenKind::Operator(String::from(byte as char))),
                b'0'..=b'9' => Ok(self.lex_int(byte)),
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => Ok(self.lex_word(byte)),
                bad_token => Err(new_error(
                    format!("bad token {}", bad_token as char).as_str(),
                )),
            }?;
            tokens.push(token)
        }
        Ok(tokens)
    }

    fn lex_int(&mut self, byte: u8) -> TokenKind {
        let mut int = String::from(byte as char);
        loop {
            let next = match self.chars.peek() {
                Some(byte) => byte,
                None => break,
            };
            match next {
                b'0'..=b'9' => int.push(self.chars.next().unwrap() as char),
                _ => break,
            }
        }
        TokenKind::Int(int)
    }

    fn lex_word(&mut self, byte: u8) -> TokenKind {
        let mut word = String::from(byte as char);
        loop {
            let next = match self.chars.peek() {
                Some(byte) => byte,
                None => break Self::lex_keyword(&word),
            };
            match next {
                b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    word.push(self.chars.next().unwrap() as char);
                }
                b'(' => {
                    word.push(self.chars.next().unwrap() as char);
                    break TokenKind::Callable(word);
                }
                _ => break Self::lex_keyword(&word),
            }
        }
    }

    fn lex_keyword(word: &str) -> TokenKind {
        match word {
            "for" => TokenKind::For,
            "rof" => TokenKind::EndFor,
            "while" => TokenKind::While,
            "elihw" => TokenKind::EndWhile,
            "in" => TokenKind::In,
            "exit" => TokenKind::Exit,
            _ => TokenKind::VarName(word.to_string()),
        }
    }
}
