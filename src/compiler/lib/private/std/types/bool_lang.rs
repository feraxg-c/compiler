
use crate::compiler::lib::private::std::types::type_compress::TraitTypeFn;

pub struct BoolLang {
    pub val: bool,
}

impl BoolLang {
    pub fn new(val: bool) -> BoolLang {
        BoolLang { val }
    }

    pub fn convert_bool_to_int(&self) -> i8 {
        if self.val {
            1
        } else {
            0
        }
    }
}

impl TraitTypeFn for BoolLang {
    fn convert_type_to_c(&self) -> String {
        "Bool".to_string() // Возвращаем строку для типа
    }

    fn create_c_variable(&self, name: String) -> String {
        format!(
            "{} {} = createBool({})",
            self.convert_type_to_c(),
            name,
            self.convert_bool_to_int()
        )
    }
}


