pub enum Token{
    StartLZone,         // {
    StartRZone,         // }
    Add,                // +
    Subtract,           // -
    Multiply,           // *
    Slash,              // /
    Identifier,         // aZ..Az | аЯ..Ая
    Semicolon,          // ;
    Comma,              // ,
    Lparen,             // (
    Rparen,             // )
    Colon,              // :
    Space,              // ` `
    EndLine,            // /r|/t
}