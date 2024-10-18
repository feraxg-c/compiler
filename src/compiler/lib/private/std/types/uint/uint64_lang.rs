
pub struct IntU64Lang{
    val: u32
}

impl IntU64Lang {
    pub(crate) fn new(val: u32) -> IntU64Lang {
        IntU64Lang {
            val
        }
    }

    fn convert_type_to_c(&self) -> String {
        "UInt32".to_string()
    }
}
