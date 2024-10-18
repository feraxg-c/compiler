
pub struct CharLang {
    pub(crate) val: char
}

impl CharLang {
    pub fn new(val: char) -> CharLang {
        CharLang {
            val
        }
    }

    pub fn convert_type_to_c() -> String {
        "char".to_string()
    }
}
