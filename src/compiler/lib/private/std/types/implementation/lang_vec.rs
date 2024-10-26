use crate::compiler::lib::private::std::types::type_compress::{Types, Value, ValueData};

pub struct LangVec{
    value: Vec<ValueData>,
    data: Types,
}

impl LangVec {
    pub fn new(value: Vec<ValueData>, data: Types) -> LangVec{
        LangVec{
            value: value.clone(),
            data,
        }
    }

    pub fn element_push(&mut self, element: Value){
        if element.data_type == self.data {
            self.value.push(element.value)
        }else {
            // TODO: call Error
        }
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }
}
