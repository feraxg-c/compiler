use crate::compiler::lib::private::std::types::type_compress::TraitTypeFn;

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

}

impl TraitTypeFn for StringLang{
    fn convert_type_to_c(&self) -> String {
        "String".to_string()
    }

    fn create_c_variable(&self, name: String) -> String {
        format!(
            "{} {} = createString({})",
            Self::convert_type_to_c(&self),
            name, &self.val
        )
    }
}

