use std::iter::Peekable;
use std::slice::Iter;
use crate::compiler::lexer::token;
use crate::compiler::lexer::token::Token;
use crate::compiler::parser::ast::Expr;

fn ast_parser(tokens: Vec<Token>) -> Vec<Expr> {
    let mut iter = tokens.iter().peekable();
    let mut ast_tree: Vec<Expr> = vec![];

    while let Some(current) = iter.next() {
        match current {
            Token::Number(value) => {
                if let Ok(num) = value.value.parse::<i64>() {
                    ast_tree.push(Expr::Number(num));
                }
            }
            Token::Identifier(value) => {
                let mut fn_name: &str;
                if value.value == "ptr" {
                    iter.next();
                    if value.value == "\""{
                        iter.next();

                    }
                }
            }
            Token::Add | Token::Subtract | Token::Multiply | Token::Slash => {
                if let Some(left) = ast_tree.pop() {
                    if let Some(right_token) = iter.next() {
                        let right_expr = match right_token {
                            Token::Number(value) => {
                                if let Ok(num) = value.value.parse::<i64>() {
                                    Expr::Number(num)
                                } else {
                                    continue;
                                }
                            }
                            Token::Identifier(name) => Expr::Identifier(name.value.clone()),
                            _ => continue,
                        };
                        let operation = match current {
                            Token::Add => "+".to_string(),
                            Token::Subtract => "-".to_string(),
                            Token::Multiply => "*".to_string(),
                            Token::Slash => "/".to_string(),
                            _ => unreachable!(),
                        };
                        ast_tree.push(Expr::BinaryOp(Box::new(left), operation, Box::new(right_expr)));
                    }
                }
            }
            Token::StartLZone => {}
            Token::StartRZone => {}
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
