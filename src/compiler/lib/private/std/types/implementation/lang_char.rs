#[derive(Clone)]
pub struct LangChar{
    pub value: char
}


impl LangChar{
    pub fn new(value: char) -> LangChar{
        LangChar{
            value
        }
    }
}