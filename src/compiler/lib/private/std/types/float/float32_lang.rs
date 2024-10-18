
pub struct Float32Lang{
    val: f32
}

impl Float32Lang {
    pub(crate) fn new(val: f32) -> Float32Lang {
        Float32Lang {
            val
        }
    }

    fn convert_type_to_c(&self) -> String {
        "Float32".to_string()
    }
}
