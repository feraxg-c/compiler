use crate::compiler::lib::private::std::types::implementation::lang_boolean::LangBoolean;
use crate::compiler::lib::private::std::types::implementation::lang_char::LangChar;
use crate::compiler::lib::private::std::types::implementation::lang_floats::{LangFloat32, LangFloat64};
use crate::compiler::lib::private::std::types::implementation::lang_list::LangList;
use crate::compiler::lib::private::std::types::implementation::lang_string::LangString;
use crate::compiler::lib::private::std::types::implementation::lang_ints::
{
    LangInt16,
    LangInt32,
    LangInt64,
    LangInt8
};
use crate::compiler::lib::private::std::types::implementation::lang_struct::LangStruct;
use crate::compiler::lib::private::std::types::implementation::lang_uints::
{
    LangUInt16,
    LangUInt32,
    LangUInt64,
    LangUInt8
};
use crate::compiler::lib::private::std::types::implementation::lang_vec::LangVec;

#[derive(PartialEq, Clone)]
pub enum Types{
    LangStruct,
    LangBoolean,
    LangList,
    LangVec,
    LangString,
    LangChar,
    LangInt8,
    LangInt16,
    LangInt32,
    LangInt64,
    LangUInt8,
    LangUInt16,
    LangUInt32,
    LangUInt64,
    LangFloat32,
    LangFloat64,
}
#[derive(Clone)]
pub enum ValueData{
    LangStruct(LangStruct),
    LangBoolean(LangBoolean),
    LangList(LangList),
    LangVec(LangVec),
    LangString(LangString),
    LangChar(LangChar),
    LangInt8(LangInt8),
    LangInt16(LangInt16),
    LangInt32(LangInt32),
    LangInt64(LangInt64),
    LangUInt8(LangUInt8),
    LangUInt16(LangUInt16),
    LangUInt32(LangUInt32),
    LangUInt64(LangUInt64),
    LangFloat32(LangFloat32),
    LangFloat64(LangFloat64),
}
#[derive(Clone)]
pub struct ParameterType {
    name: String,
    data: Types
}
#[derive(Clone)]
pub struct Value {
    pub data_type: Types,
    pub value: ValueData,
}






