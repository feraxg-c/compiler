<<<<<<< HEAD
struct IntLang{
    val: i64
}

impl IntLang{
    fn new(val:i64) -> IntLang {
        IntLang{
            val
        }
    }

    fn convert_type_to_c(&self) -> String{
        "int".to_string()
    }

=======
struct IntLang{
    val: i64
}

impl IntLang{
    fn new(val:i64) -> IntLang {
        IntLang{
            val
        }
    }

    fn convert_type_to_c(&self) -> String{
        "int".to_string()
    }

>>>>>>> 0b75cfab27fcd10ead4c1faef6dc94aa15a84199
}