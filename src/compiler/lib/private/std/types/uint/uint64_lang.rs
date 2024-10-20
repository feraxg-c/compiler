use crate::compiler::lib::private::std::types::type_compress::TraitTypeFn;

pub struct IntU64Lang{
    val: u64
}

impl IntU64Lang {
    pub(crate) fn new(val: u64) -> IntU64Lang {
        IntU64Lang {
            val
        }
    }
}

impl TraitTypeFn for IntU64Lang{
    fn convert_type_to_c(&self) -> String {
        "UInt64".to_string()
    }

    fn create_c_variable(&self, name: String) -> String {
        format!(
            "{} {} = createUInt64({})",
            Self::convert_type_to_c(self),
            name,
            self.val
        )
    }
}
