use crate::compiler::lib::private::std::types::type_compress::{Types, ValueData};

#[derive(Clone)]
pub struct LangList{
    value: Vec<ValueData>,
    data: Types,
    len: usize,
}

impl LangList {
    pub fn new(value: Vec<ValueData>, data: Types, len: usize) -> LangList{
        LangList{
            value: value.clone(),
            data,
            len
        }
    }
}
