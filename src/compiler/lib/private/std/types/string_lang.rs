
pub struct StringLang {
    val: String,
    len: usize,
}

impl StringLang {
    pub fn new(val: String) -> StringLang {
        StringLang {
            val: val.clone(), // Клонируем строку
            len: val.len(),   // Длина строки в usize
        }
    }

    fn convert_type_to_c(&self) -> String {
        format!("char[{}]", self.len)
    }
}
