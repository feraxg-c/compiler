<<<<<<< HEAD
=======
<<<<<<< HEAD
>>>>>>> 9c061fd5b3ab1734c3f3d1c060b92ee7df528ba4
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub enum Token {
    Identifier(LexerValueString), // aZ..Az | аЯ..Ая
    Number(LexerValueString),     // 0..9
    StartLZone,                   // {
    StartRZone,                   // }
    Add,                          // +
    Subtract,                     // -
    Multiply,                     // *
    Slash,                        // /
    Semicolon,                    // ;
    Comma,                        // ,
    Lparen,                       // (
    Rparen,                       // )
    Colon,                        // :
    ExclamationMark,              // !
    EndLine(LexerValueChar),      // \r\n | \t
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



<<<<<<< HEAD
=======
=======
pub enum Token {
    Identifier(LexerValueString), // aZ..Az | аЯ..Ая
    Number(LexerValueString),     // 0..9
    StartLZone,                   // {
    StartRZone,                   // }
    Add,                          // +
    Subtract,                     // -
    Multiply,                     // *
    Slash,                        // /
    Semicolon,                    // ;
    Comma,                        // ,
    Lparen,                       // (
    Rparen,                       // )
    Colon,                        // :
    ExclamationMark,              // !
    EndLine(LexerValueChar),      // \r\n | \t
}


pub struct LexerValueString {
    pub(crate) value: String,
}

impl LexerValueString {
    pub fn new(value: String) -> Self {
        LexerValueString { value }
    }
}

pub struct LexerValueChar {
    pub(crate) value: char,
}

impl LexerValueChar {
    pub fn new(value: char) -> Self {
        LexerValueChar { value }
    }
}
>>>>>>> 0b75cfab27fcd10ead4c1faef6dc94aa15a84199
>>>>>>> 9c061fd5b3ab1734c3f3d1c060b92ee7df528ba4
