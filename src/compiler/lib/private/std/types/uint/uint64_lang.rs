
pub struct IntU64Lang{
    val: u64
}

impl IntU64Lang {
    pub(crate) fn new(val: u64) -> IntU64Lang {
        IntU64Lang {
            val
        }
    }

    fn convert_type_to_c(&self) -> String {
        "UInt64".to_string()
    }
}
