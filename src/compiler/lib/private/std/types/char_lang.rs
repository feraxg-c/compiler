
pub struct CharLang {
    val: char
}

impl CharLang {
    fn new(val: char) -> CharLang {
        CharLang {
            val
        }
    }

    pub fn convert_type_to_c() -> String {
        "char".to_string()
    }
}
