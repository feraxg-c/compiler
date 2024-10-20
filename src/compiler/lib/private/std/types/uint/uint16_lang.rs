use crate::compiler::lib::private::std::types::type_compress::TraitTypeFn;
use crate::compiler::lib::private::std::types::uint::uint8_lang::IntU8Lang;

pub struct IntU16Lang{
    val: u16
}

impl IntU16Lang {
    pub(crate) fn new(val: u16) -> IntU16Lang {
        IntU16Lang {
            val
        }
    }

}

impl TraitTypeFn for IntU16Lang{
    fn convert_type_to_c(&self) -> String {
        "UInt16".to_string()
    }

    fn create_c_variable(&self, name: String) -> String {
        format!(
            "{} {} = createUInt16({})",
            Self::convert_type_to_c(self),
            name,
            self.val
        )
    }
}
