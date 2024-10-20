use crate::compiler::lib::private::std::types::type_compress::TraitTypeFn;

pub struct BoolLang{
    pub val: bool
}

impl BoolLang {
    pub fn new(val: bool) -> BoolLang {
        BoolLang {
            val
        }
    }

}

impl TraitTypeFn for BoolLang{
    fn convert_type_to_c(&self) -> String {
        "Bool".to_string() // typedef struct
    }

    fn create_c_variable(&self, name: String) -> String {
        format!("{} {} = createBool({})", Self::convert_type_to_c(&self), name, self.val)
    }
}

