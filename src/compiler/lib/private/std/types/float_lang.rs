<<<<<<< HEAD
=======
<<<<<<< HEAD
>>>>>>> 9c061fd5b3ab1734c3f3d1c060b92ee7df528ba4
struct Float64Lang {
    val: f64
}

impl Float64Lang {
    fn new(val:f64) -> Float64Lang {
        Float64Lang {
            val
        }
    }

    fn convert_type_to_c(&self) -> String{
        "Float64".to_string()
    }

    

<<<<<<< HEAD
=======
=======
struct FloatLang{
    val: f64
}

impl FloatLang{
    fn new(val:f64) -> FloatLang {
        FloatLang{
            val
        }
    }

    fn convert_type_to_c(&self) -> String{
        "float".to_string()
    }

>>>>>>> 0b75cfab27fcd10ead4c1faef6dc94aa15a84199
>>>>>>> 9c061fd5b3ab1734c3f3d1c060b92ee7df528ba4
}