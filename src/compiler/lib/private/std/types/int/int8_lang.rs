
pub struct Int8Lang{
    val: i8
}

impl Int8Lang {
    pub(crate) fn new(val: i8) -> Int8Lang {
        Int8Lang {
            val
        }
    }

    fn convert_type_to_c(&self) -> String {
        "Int8".to_string()
    }
}
