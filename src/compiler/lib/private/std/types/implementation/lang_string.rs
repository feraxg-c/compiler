#[derive(Clone)]
pub struct LangString{
    pub value: String
}

impl LangString{
    fn new(value: String) -> LangString{
        LangString{
            value
        }
    }
}