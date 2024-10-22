use crate::compiler::lib::private::std::io::arg::argument::Argument;
use crate::compiler::lib::private::std::types::bool_lang::BoolLang;
use crate::compiler::lib::private::std::types::char_lang::CharLang;
use crate::compiler::lib::private::std::types::float::float32_lang::Float32Lang;
use crate::compiler::lib::private::std::types::float::float64_lang::Float64Lang;
use crate::compiler::lib::private::std::types::int::int16_lang::Int16Lang;
use crate::compiler::lib::private::std::types::int::int32_lang::Int32Lang;
use crate::compiler::lib::private::std::types::int::int64_lang::Int64Lang;
use crate::compiler::lib::private::std::types::int::int8_lang::Int8Lang;
use crate::compiler::lib::private::std::types::string_lang::StringLang;
use crate::compiler::lib::private::std::types::struct_lang::StructLang;
use crate::compiler::lib::private::std::types::uint::uint16_lang::IntU16Lang;
use crate::compiler::lib::private::std::types::uint::uint32_lang::IntU32Lang;
use crate::compiler::lib::private::std::types::uint::uint64_lang::IntU64Lang;
use crate::compiler::lib::private::std::types::uint::uint8_lang::IntU8Lang;

pub trait TraitTypeFn {
    fn convert_type_to_c(&self) -> String;
    fn create_c_variable(&self, name: String) -> String;
}

pub enum LangType {
    Bool(bool),
    Char(char),
    MyString(String),
    Class,
    Enum,
    Float32(f32),
    Float64(f64),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    MyStruct(String, Vec<Argument>),
    None(Option<LangType>),
}

impl LangType {
    pub(crate) fn convert_type_to_struct(&self) -> Box<dyn std::any::Any> {
        match &self {
            LangType::Bool(val) => Box::new(BoolLang::new(*val)),
            LangType::Char(val) => Box::new(CharLang::new(*val)),
            LangType::MyString(val) => Box::new(StringLang::new(val.clone())),
            LangType::Float32(val) => Box::new(Float32Lang::new(*val)),
            LangType::Float64(val) => Box::new(Float64Lang::new(*val)),
            LangType::Int8(val) => Box::new(Int8Lang::new(*val)),
            LangType::Int16(val) => Box::new(Int16Lang::new(*val)),
            LangType::Int32(val) => Box::new(Int32Lang::new(*val)),
            LangType::Int64(val) => Box::new(Int64Lang::new(*val)),
            LangType::UInt8(val) => Box::new(IntU8Lang::new(*val)),
            LangType::UInt16(val) => Box::new(IntU16Lang::new(*val)),
            LangType::UInt32(val) => Box::new(IntU32Lang::new(*val)),
            LangType::UInt64(val) => Box::new(IntU64Lang::new(*val)),
            LangType::MyStruct(name, arg) => Box::new(StructLang::new(name.clone(), arg.clone())),
            _ => Box::new(()), // Возвращаем пустой блок для других типов
        }
    }

    pub fn convert_type_to_c(&self) -> String {
        match self {
            LangType::Bool(val) => BoolLang::new(*val).convert_type_to_c(),
            LangType::Char(val) => CharLang::new(*val).convert_type_to_c(),
            LangType::MyString(val) => StringLang::new(val.clone()).convert_type_to_c(),
            LangType::Float32(val) => Float32Lang::new(*val).convert_type_to_c(),
            LangType::Float64(val) => Float64Lang::new(*val).convert_type_to_c(),
            LangType::Int8(val) => Int8Lang::new(*val).convert_type_to_c(),
            LangType::Int16(val) => Int16Lang::new(*val).convert_type_to_c(),
            LangType::Int32(val) => Int32Lang::new(*val).convert_type_to_c(),
            LangType::Int64(val) => Int64Lang::new(*val).convert_type_to_c(),
            LangType::UInt8(val) => IntU8Lang::new(*val).convert_type_to_c(),
            LangType::UInt16(val) => IntU16Lang::new(*val).convert_type_to_c(),
            LangType::UInt32(val) => IntU32Lang::new(*val).convert_type_to_c(),
            LangType::UInt64(val) => IntU64Lang::new(*val).convert_type_to_c(),

            _ => String::new(), // обработка других типов
        }
    }


    pub(crate) fn create_c_variable(&self, name: String) -> String {
        match self {
            LangType::Bool(val) => BoolLang::new(*val).create_c_variable(name),
            LangType::Char(val) => CharLang::new(*val).create_c_variable(name),
            LangType::MyString(val) => StringLang::new(val.clone()).create_c_variable(name),
            LangType::Float32(val) => Float32Lang::new(*val).create_c_variable(name),
            LangType::Float64(val) => Float64Lang::new(*val).create_c_variable(name),
            LangType::Int8(val) => Int8Lang::new(*val).create_c_variable(name),
            LangType::Int16(val) => Int16Lang::new(*val).create_c_variable(name),
            LangType::Int32(val) => Int32Lang::new(*val).create_c_variable(name),
            LangType::Int64(val) => Int64Lang::new(*val).create_c_variable(name),
            LangType::UInt8(val) => IntU8Lang::new(*val).create_c_variable(name),
            LangType::UInt16(val) => IntU16Lang::new(*val).create_c_variable(name),
            LangType::UInt32(val) => IntU32Lang::new(*val).create_c_variable(name),
            LangType::UInt64(val) => IntU64Lang::new(*val).create_c_variable(name),

            _ => String::new(), // обработка других типов
        }
    }


}
