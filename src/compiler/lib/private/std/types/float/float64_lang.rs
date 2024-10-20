use crate::compiler::lib::private::std::types::float::float32_lang::Float32Lang;
use crate::compiler::lib::private::std::types::type_compress::TraitTypeFn;

pub struct Float64Lang {
    val: f64
}

impl Float64Lang {
    pub(crate) fn new(val: f64) -> Float64Lang {
        Float64Lang {
            val
        }
    }

    fn convert_type_to_c(&self) -> String {
        "Float64".to_string()
    }
}

impl TraitTypeFn for  Float64Lang {
    fn convert_type_to_c(&self) -> String {
        "Float64".to_string()
    }

    fn create_c_variable(&self, name: String) -> String {
        format!(
            "{} {} = createFloat64({})",
            Self::convert_type_to_c(self),
            name,
            self.val
        )
    }
}
    


