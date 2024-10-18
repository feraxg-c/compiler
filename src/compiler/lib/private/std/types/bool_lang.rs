

pub struct BoolLang{
    pub val: bool
}

impl BoolLang {
    pub fn new(val: bool) -> BoolLang {
        BoolLang {
            val
        }
    }
    pub fn convert_type_to_c(&self) -> String {
        "Bool".to_string() // typedef struct
    }

    pub fn print_boolean_value() -> String {
        format!("printBool({})", 1)
    }
}

