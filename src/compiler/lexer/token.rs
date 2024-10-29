use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub enum Token {
    /// aZ..Az | аЯ..Ая | yX..Xy
    Identifier(LexerValueString),
    /// 0..9
    Number(LexerValueString),
    /// 0.0--9.(9)
    NumberFloat(LexerValueString),
    /// {
    StartLZone,
    /// }
    StartRZone,
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Multiply,
    /// /
    Slash,
    /// ;
    Semicolon,
    /// ,
    Comma,
    /// (
    Lparen,
    /// )
    Rparen,
    /// :
    Colon,
    /// !
    ExclamationMark,
    /// \r\n | \t
    EndLine(LexerValueChar),
    /// "
    DoubleQuotes,
    /// '
    OneQuotes
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



