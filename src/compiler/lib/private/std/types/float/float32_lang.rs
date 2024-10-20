use crate::compiler::lib::private::std::types::type_compress::TraitTypeFn;

pub struct Float32Lang{
    val: f32
}

impl Float32Lang {
    pub fn new(val: f32) -> Float32Lang {
        Float32Lang {
            val
        }
    }

}

impl TraitTypeFn for  Float32Lang {
    fn convert_type_to_c(&self) -> String {
        "Float32".to_string()
    }

    fn create_c_variable(&self, name: String) -> String {
        format!(
            "{} {} = createFloat32({})",
            Self::convert_type_to_c(self),
            name,
            self.val
        )
    }
}