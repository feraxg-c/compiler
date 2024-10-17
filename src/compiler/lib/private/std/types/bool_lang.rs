<<<<<<< HEAD
=======
<<<<<<< HEAD
>>>>>>> 9c061fd5b3ab1734c3f3d1c060b92ee7df528ba4

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

<<<<<<< HEAD
=======
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
>>>>>>> 9c061fd5b3ab1734c3f3d1c060b92ee7df528ba4
}