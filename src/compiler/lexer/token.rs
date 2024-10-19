use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub enum Token {
    Identifier(LexerValueString),   // aZ..Az | аЯ..Ая
    Number(LexerValueString),       // 0..9
    NumberFloat(LexerValueString),  // 0.0--9.(9)
    StartLZone,                     // {
    StartRZone,                     // }
    Add,                            // +
    Subtract,                       // -
    Multiply,                       // *
    Slash,                          // /
    Semicolon,                      // ;
    Comma,                          // ,
    Lparen,                         // (
    Rparen,                         // )
    Colon,                          // :
    ExclamationMark,                // !
    EndLine(LexerValueChar),        // \r\n | \t
    DoubleQuotes,                   // "
    OneQuotes                       // '
}
#[derive(Debug)]
pub struct LexerValueString {
    pub(crate) value: String,
}

impl LexerValueString {
    pub fn new(value: String) -> Self {
        LexerValueString { value }
    }
}

#[derive(Debug)]
pub struct LexerValueChar {
    pub(crate) value: char,
}


impl LexerValueChar {
    pub fn new(value: char) -> Self {
        LexerValueChar { value }
    }
}



