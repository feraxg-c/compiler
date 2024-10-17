<<<<<<< HEAD

struct BoolLang{
    val: bool
}

impl BoolLang{
    fn new(val: bool) -> BoolLang {
        BoolLang{
            val
        }
    }
    fn convert_type_to_c(&self) -> String{
        "Bool".to_string() // typedef struct
    }

    fn print_boolean_value() -> String{
        format!("printBool({})", 1)
    }

=======

struct BoolLang{
    val: bool
}

impl BoolLang{
    fn new(val: bool) -> BoolLang {
        BoolLang{
            val
        }
    }
    fn convert_type_to_c(&self) -> String{
        "Bool".to_string() // typedef struct
    }

>>>>>>> 0b75cfab27fcd10ead4c1faef6dc94aa15a84199
}