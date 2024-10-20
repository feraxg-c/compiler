use crate::compiler::lib::private::std::types::type_compress::TraitTypeFn;

pub struct Int32Lang{
    val: i32
}

impl Int32Lang {
    pub(crate) fn new(val: i32) -> Int32Lang {
        Int32Lang {
            val
        }
    }
}

impl TraitTypeFn for Int32Lang{
    fn convert_type_to_c(&self) -> String {
        "Int32".to_string()
    }

    fn create_c_variable(&self, name: String) -> String {
        format!(
            "{} {} = createInt32({})",
            Self::convert_type_to_c(self),
            name,
            self.val
        )
    }
}
