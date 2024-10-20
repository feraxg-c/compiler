use crate::compiler::lib::private::std::types::type_compress::TraitTypeFn;
use crate::compiler::lib::private::std::types::uint::uint8_lang::IntU8Lang;

pub struct Int8Lang{
    val: i8
}

impl Int8Lang {
    pub(crate) fn new(val: i8) -> Int8Lang {
        Int8Lang {
            val
        }
    }

}

impl TraitTypeFn for Int8Lang{
    fn convert_type_to_c(&self) -> String {
        "Int8".to_string()
    }

    fn create_c_variable(&self, name: String) -> String {
        format!(
            "{} {} = createInt8({})",
            Self::convert_type_to_c(self),
            name,
            self.val
        )
    }
}
