use crate::compiler::lib::private::std::types::type_compress::ParameterType;

#[derive(Clone)]
pub struct LangStruct{
    value: Vec<ParameterType>
}

impl LangStruct{
    pub fn new(value: Vec<ParameterType>) -> LangStruct {
        LangStruct{
            value
        }
    }
}