
struct IntLang{
    val: i8
}

impl IntLang {
    fn new(val: i8) -> IntLang {
        IntLang {
            val
        }
    }

    fn convert_type_to_c(&self) -> String {
        "Int8".to_string()
    }
}
