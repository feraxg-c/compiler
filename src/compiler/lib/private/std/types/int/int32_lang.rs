
pub struct Int32Lang{
    val: i32
}

impl Int32Lang {
    pub(crate) fn new(val: i32) -> Int32Lang {
        Int32Lang {
            val
        }
    }

    fn convert_type_to_c(&self) -> String {
        "Int32".to_string()
    }
}
