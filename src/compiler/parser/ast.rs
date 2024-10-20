
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
    Number(i64),                                    // 0..9
    NumberFloat(f64),                               // 0.0--9.(9)
    BooleanLiteral(bool),                           // true, false
    Identifier(String),                             // name, variable..other
    UnaryOp(Box<Expr>, String),                     // !true, i++
    BinaryOp(Box<Expr>, String, Box<Expr>),         // branches tree
    FunctionCall(String, Vec<Expr>, Box<Expr>),     // funcName() 1 - name 2 - arguments 3 - return val
    Group(Box<Expr>),                               // group Expr
    Assignment(String, Box<Expr>),                  // for variable
    StartLZone,                                     // {
    StartRZone,                                     // }
    Semicolon
}

impl Expr {
    fn evaluate(&self) -> i64 {
        match self {
            Expr::Number(value) => *value,
            Expr::BooleanLiteral(_) => {
                0
            }
            Expr::BinaryOp(left, op, right) => {
                let left_value = left.evaluate();
                let right_value = right.evaluate();

                match op.as_str() {
                    "+" => left_value + right_value,
                    "-" => left_value - right_value,
                    "*" => left_value * right_value,
                    "/" => left_value / right_value, // добавлено деление
                    "!=" => if left_value != right_value { 1 } else { 0 }, // non equals
                    "==" => if left_value == right_value { 1 } else { 0 }, // equals
                    ">" => if left_value > right_value { 1 } else { 0 }, // more
                    "<" => if left_value < right_value { 1 } else { 0 }, // small
                    _ => 0, // TODO: call error
                }
            }
            // processing other operand
            _ => 0,
        }
    }

    pub fn new_binary_op(ast_tree: &mut Vec<Expr>, operand: String) {
        // if there is left operand, walk next
        if let Some(left) = ast_tree.pop() {
            // if there is right operand, walk next
            if let Some(right) = ast_tree.pop() {
                ast_tree.push(Expr::BinaryOp(Box::new(left), operand, Box::new(right)));
            }
        }
    }
}