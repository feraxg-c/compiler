use crate::compiler::lib::private::std::types::type_compress::{TraitTypeFn};

pub struct CharLang {
    pub(crate) val: char
}

impl CharLang {
    pub fn new(val: char) -> CharLang {
        CharLang {
            val
        }
    }
}

impl TraitTypeFn for CharLang{
    fn convert_type_to_c(&self) -> String {
        "СharLang".to_string()
    }

    fn create_c_variable(&self, name: String) -> String {
        format!("{} {} = createCharLang('{}')", Self::convert_type_to_c(&self), name, self.val)
    }
}
