
pub struct Int16Lang{
    val: i16
}

impl Int16Lang {
    pub fn new(val: i16) -> Int16Lang {
        Int16Lang {
            val
        }
    }

    fn convert_type_to_c(&self) -> String {
        "Int16".to_string()
    }
}
