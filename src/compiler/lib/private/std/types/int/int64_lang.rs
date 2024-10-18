
struct IntLang{
    val: i64
}

impl IntLang {
    fn new(val: i64) -> IntLang {
        IntLang {
            val
        }
    }

    fn convert_type_to_c(&self) -> String {
        "Int64".to_string()
    }
}