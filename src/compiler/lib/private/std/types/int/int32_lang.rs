
struct IntLang{
    val: i32
}

impl IntLang {
    fn new(val: i32) -> IntLang {
        IntLang {
            val
        }
    }

    fn convert_type_to_c(&self) -> String {
        "Int32".to_string()
    }
}
