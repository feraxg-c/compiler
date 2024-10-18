
pub struct IntU16Lang{
    val: u16
}

impl IntU16Lang {
    pub(crate) fn new(val: u16) -> IntU16Lang {
        IntU16Lang {
            val
        }
    }

    fn convert_type_to_c(&self) -> String {
        "UInt16".to_string()
    }
}
