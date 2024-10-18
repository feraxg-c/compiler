
pub struct Int64Lang{
    val: i64
}

impl Int64Lang {
    pub(crate) fn new(val: i64) -> Int64Lang {
        Int64Lang {
            val
        }
    }

    fn convert_type_to_c(&self) -> String {
        "Int64".to_string()
    }
}
