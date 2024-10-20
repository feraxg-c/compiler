use crate::compiler::lib::private::std::types::type_compress::TraitTypeFn;
use crate::compiler::lib::private::std::types::uint::uint8_lang::IntU8Lang;

pub struct IntU32Lang{
    val: u32
}

impl IntU32Lang {
    pub(crate) fn new(val: u32) -> IntU32Lang {
        IntU32Lang {
            val
        }
    }


}

impl TraitTypeFn for IntU32Lang{
    fn convert_type_to_c(&self) -> String {
        "UInt32".to_string()
    }

    fn create_c_variable(&self, name: String) -> String {
        format!(
            "{} {} = createUInt32({})",
            Self::convert_type_to_c(self),
            name,
            self.val
        )
    }
}
