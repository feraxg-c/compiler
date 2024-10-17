use crate::compiler::lexer::token::Token;
use crate::compiler::parser::ast::Expr;

fn parser_to_ast(tokens: Vec<Token>) -> Vec<Expr> {
    let mut expr: Vec<Expr> = vec![];
    let mut current_expr: Option<Expr> = None;

    for token in tokens.iter().rev() {
        match token {
            Token::Identifier(lexer_val_str) => {
                if lexer_val_str.value == "fn" {
                    // Обработка определения функции
                } else {
                    current_expr = Some(Expr::Identifier(lexer_val_str.value.clone()));
                }
            }
            Token::Number(lexer_value_string) => {
                // Преобразуем строку в i64
                if let Ok(value) = lexer_value_string.value.parse::<i64>() {
                    current_expr = Some(Expr::Number(value));
                } else {
                    // Обработка ошибки, если строка не может быть преобразована в i64
                    eprintln!("Error: Unable to parse number: {}", lexer_value_string.value);
                }
            }
            Token::Add => {
                if let (Some(left), Some(right)) = (expr.pop(), current_expr.take()) {
                    current_expr = Some(Expr::BinaryOperation(Box::new(left), "+".to_string(), Box::new(right)));
                }
            }
            Token::Subtract => {
                if let (Some(left), Some(right)) = (expr.pop(), current_expr.take()) {
                    current_expr = Some(Expr::BinaryOperation(Box::new(left), "-".to_string(), Box::new(right)));
                }
            }
            Token::Multiply => {
                if let (Some(left), Some(right)) = (expr.pop(), current_expr.take()) {
                    current_expr = Some(Expr::BinaryOperation(Box::new(left), "*".to_string(), Box::new(right)));
                }}
            Token::Slash => {
                if let (Some(left), Some(right)) = (expr.pop(), current_expr.take()) {
                    current_expr = Some(Expr::BinaryOperation(Box::new(left), "/".to_string(), Box::new(right)));
                }
            }
            Token::Semicolon => {
                if let Some(e) = current_expr.take() {
                    expr.push(e);
                }
            }
            Token::Lparen => {
                // Обработка левой круглой скобки
            }
            Token::Rparen => {
                // Обработка правой круглой скобки
            }
            _ => {}
        }
    }

    if let Some(e) = current_expr {
        expr.push(e);
    }

    expr // Возвращаем собранные выражения
}