use crate::compiler::lexer::token::Token;
use crate::compiler::parser::ast::Expr;

fn ast_parser(mut tokens: Vec<Token>) -> Vec<Expr> {
    // input: num, add, colon
    // output: colon, add, num
    let mut iter = tokens.iter().peekable().rev();
    let mut ast_tree: Vec<Expr> = vec![];

    while let Some(current) = iter.next() {
        match current {
            Token::Identifier(_) => {}
            Token::Number(_) => {}
            Token::StartLZone => {
                ast_tree.push(Expr::StartLZone)
            }
            Token::StartRZone => {
                ast_tree.push(Expr::StartRZone)
            }
            Token::Add => {}
            Token::Subtract => {}
            Token::Multiply => {}
            Token::Slash => {}
            Token::Semicolon => {}
            Token::Comma => {}
            Token::Lparen => {}
            Token::Rparen => {}
            Token::Colon => {}
            Token::ExclamationMark => {}
            Token::EndLine(_) => {}
            Token::DoubleQuotes => {}
            Token::OneQuotes => {}
        }
    }

    ast_tree
}
