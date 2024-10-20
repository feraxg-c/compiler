use crate::compiler::lib::private::std::types::int::int8_lang::Int8Lang;
use crate::compiler::lib::private::std::types::type_compress::TraitTypeFn;

pub struct Int16Lang{
    val: i16
}

impl Int16Lang {
    pub fn new(val: i16) -> Int16Lang {
        Int16Lang {
            val
        }
    }


}

impl TraitTypeFn for Int16Lang{
    fn convert_type_to_c(&self) -> String {
        "Int16".to_string()
    }

    fn create_c_variable(&self, name: String) -> String {
        format!(
            "{} {} = createInt16({})",
            Self::convert_type_to_c(self),
            name,
            self.val
        )
    }
}
