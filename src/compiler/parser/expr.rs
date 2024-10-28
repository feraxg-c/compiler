use crate::compiler::lib::private::std::types::type_compress::ParameterType;

pub enum Expr {

    // Литералы
    Number(i64),                 // Числовые литералы
    Boolean(bool),               // Логические литералы
    StringLiteral(String),       // Строковые литералы
    CharLiteral(char),           // Символьные литералы

    // Операции
    Binary(Box<Expr>, Operator, Box<Expr>), // Бинарные операции
    Unary(Operator, Box<Expr>),             // Унарные операции

    // Переменные
    Variable(String),            // Переменные

    // Вызов переменных, функций, ...
    SomeCall(String), // Вызов чего-либо с аргументами

    // Условные выражения
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>), // if условие, then блок, else блок

    // Циклы
    Loop(Box<Expr>),             // Циклы, например, while

}





pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    And,
    Or,
    Not,
}