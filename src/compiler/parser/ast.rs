use crate::compiler::lexer::token::Token;

///
///```
///       2 + 2
///       /  \
/// val1 /op:+\ val
///     2      2
///     --------
///        4
/// ```
///```
/// a = 1
/// b = 2
/// if a != b
/// syntax tree:
///    a != b
///   /      \
///  /op:`!=` \
/// 1          2
/// ------------
///     true
///```
#[derive(Debug)]
pub enum Expr {
    Number(i64),                                    // 34,32,12,1,9,56
    BooleanLiteral(bool),                           // true, false
    Identifier(String),                             // skldf, asdsd
    UnaryOperation(Box<Expr>, String),              // !true, i++
    BinaryOperation(Box<Expr>, String, Box<Expr>),  // ветви дерева
    FunctionCall(String, Vec<Expr>),                // funcName()
    Group(Box<Expr>),                               // группа Expr
    Assignment(String, Box<Expr>),                  // для переменных


}

