use crate::compiler::lib::private::std::types::type_compress::TraitTypeFn;

pub struct IntU8Lang{
    val: u8
}

impl IntU8Lang {
    pub(crate) fn new(val: u8) -> IntU8Lang {
        IntU8Lang {
            val
        }
    }

}

impl TraitTypeFn for IntU8Lang{
    fn convert_type_to_c(&self) -> String {
        "UInt8".to_string()
    }

    fn create_c_variable(&self, name: String) -> String {
        format!(
            "{} {} = createUInt8({})",
            Self::convert_type_to_c(self),
            name,
            self.val
        )
    }
}
