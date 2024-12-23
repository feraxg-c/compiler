use std::iter::Peekable;
use std::str::Chars;
use crate::compiler::lexer::token::{LexerValueChar, LexerValueString, Token};

//  function for creating token list
pub fn tokenize(value: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = value.chars().peekable();

    while let Some(current) = chars.next() {
        match current {
            '{' => tokens.push(Token::StartLZone),
            '}' => tokens.push(Token::StartRZone),
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Multiply),
            '/' => tokens.push(Token::Slash),
            ';' => tokens.push(Token::Semicolon),
            ',' => tokens.push(Token::Comma),
            '(' => tokens.push(Token::Lparen),
            ')' => tokens.push(Token::Rparen),
            ':' => tokens.push(Token::Colon),
            '!' => tokens.push(Token::ExclamationMark),
            '"' => tokens.push(Token::DoubleQuotes),
            '\'' => tokens.push(Token::OneQuotes),
            '\r' | '\t' | ' ' => continue, // ignore space and line feed
            _ if current.is_numeric() => {
                tokens.push(Token::Number(collect_number(&mut chars, current)));
            },
            _ if current.is_alphanumeric() => {
                tokens.push(Token::Identifier(collect_identifier(&mut chars, current)));
            },
            _ if current.is_alphanumeric() => {
                tokens.push(Token::Identifier(collect_identifier(&mut chars, current)));
            },
            _ => {
                eprintln!("Error: Unexpected character '{}'", current);
            }
        }
    }

    tokens
}

fn collect_number(chars: &mut Peekable<Chars>, first: char) -> LexerValueString {
    let mut number_str = first.to_string();
    while let Some(&next) = chars.peek() {
        if next.is_numeric() {
            number_str.push(chars.next().unwrap());
        } else {
            break;
        }
    }
    LexerValueString::new(number_str)
}

fn collect_identifier(chars: &mut Peekable<Chars>, first: char) -> LexerValueString {
    let mut identifier_str = first.to_string();
    while let Some(&next) = chars.peek() {
        if next.is_alphanumeric() {
            identifier_str.push(chars.next().unwrap());
        } else {
            break;
        }
    }
    LexerValueString::new(identifier_str)
}
