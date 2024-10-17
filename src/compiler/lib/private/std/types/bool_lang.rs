
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

}