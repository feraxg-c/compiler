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
pub enum Expr {
    Number(i64),
    Identifier(String),
    Branching(Box<Expr>, String, Box<Expr>), // Левый операнд, оператор, правый операнд
    BinaryOperation(Box<Expr>, String, Box<Expr>)
}

