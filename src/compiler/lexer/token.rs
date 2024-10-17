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