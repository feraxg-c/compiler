use crate::compiler::lexer::token::Token;
use crate::compiler::parser::ast::Expr;

// creating A.S. Tree from token
fn ast_parser(mut tokens: Vec<Token>) -> Vec<Expr> {
    // input: num, add, colon
    // output: colon, add, num
    let mut iter = tokens.iter().peekable().rev();
    let mut ast_tree: Vec<Expr> = vec![];

    while let Some(current) = iter.next() {
        match current {
            Token::StartLZone => {ast_tree.push(Expr::StartLZone)}
            Token::StartRZone => {ast_tree.push(Expr::StartRZone)}
            Token::Number(val) => {
                // if there is val.parse return true, walk next
                if let Ok(num) = val.parse::<i64>() {
                    ast_tree.push(Expr::Number(num));
                }
            }
            Token::NumberFloat(val) => {
                // if there is val.parse return true, walk next
                if let Ok(num) = val.parse::<f64>() {
                    ast_tree.push(Expr::NumberFloat(num));
                }
            }
            Token::Add => {Expr::new_binary_op(&mut ast_tree, "+".to_string());}
            Token::Subtract => {Expr::new_binary_op(&mut ast_tree, "-".to_string());}
            Token::Multiply => {Expr::new_binary_op(&mut ast_tree, "*".to_string());}
            Token::Slash => {Expr::new_binary_op(&mut ast_tree, "/".to_string());}
            Token::Comma => {}
            Token::Lparen => {}
            Token::Rparen => {}
            Token::Colon => {}
            Token::ExclamationMark => {}
            Token::EndLine(_) => {}
            Token::DoubleQuotes => {}
            Token::OneQuotes => {}
            Token::Semicolon => {ast_tree.push(Expr::Semicolon)}
            Token::Identifier(_) => {}
        }
    }

    ast_tree
}
