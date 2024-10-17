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

}