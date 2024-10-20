use crate::compiler::lib::private::std::types::int::int32_lang::Int32Lang;
use crate::compiler::lib::private::std::types::type_compress::TraitTypeFn;

pub struct Int64Lang{
    val: i64
}

impl Int64Lang {
    pub(crate) fn new(val: i64) -> Int64Lang {
        Int64Lang {
            val
        }
    }
}

impl TraitTypeFn for Int64Lang{
    fn convert_type_to_c(&self) -> String {
        "Int64".to_string()
    }

    fn create_c_variable(&self, name: String) -> String {
        format!(
            "{} {} = createInt64({})",
            Self::convert_type_to_c(self),
            name,
            self.val
        )
    }
}
