
struct IntLang{
    val: i16
}

impl IntLang {
    fn new(val: i16) -> IntLang {
        IntLang {
            val
        }
    }

    fn convert_type_to_c(&self) -> String {
        "Int16".to_string()
    }
}
