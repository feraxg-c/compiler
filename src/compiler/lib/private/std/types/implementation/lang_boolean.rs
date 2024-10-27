#[derive(Clone)]
pub struct LangBoolean{
    pub value: bool
}


impl LangBoolean{
    fn new(value: bool) -> LangBoolean {
        LangBoolean{
            value
        }
    }
}