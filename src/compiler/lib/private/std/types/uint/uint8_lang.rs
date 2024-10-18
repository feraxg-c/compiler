
pub struct IntU8Lang{
    val: u8
}

impl IntU8Lang {
    pub(crate) fn new(val: u8) -> IntU8Lang {
        IntU8Lang {
            val
        }
    }

    fn convert_type_to_c(&self) -> String {
        "UInt8".to_string()
    }
}
