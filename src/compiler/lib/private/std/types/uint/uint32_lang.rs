
pub struct IntU32Lang{
    val: u32
}

impl IntU32Lang {
    pub(crate) fn new(val: u32) -> IntU32Lang {
        IntU32Lang {
            val
        }
    }

    fn convert_type_to_c(&self) -> String {
        "UInt32".to_string()
    }
}
