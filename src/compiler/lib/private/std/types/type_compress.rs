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
use crate::compiler::lib::private::std::types::implementation::lang_uints::
{
    LangUInt16,
    LangUInt32,
    LangUInt64,
    LangUInt8
};


#[derive(PartialEq)]
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

pub struct Value {
    pub data_type: Types,
    pub value: ValueData,
}

pub enum ValueData{
    LangStruct,
    LangBoolean(LangBoolean),
    LangList(LangList),
    LangVec(LangV),
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







