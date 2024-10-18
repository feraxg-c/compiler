
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

    


