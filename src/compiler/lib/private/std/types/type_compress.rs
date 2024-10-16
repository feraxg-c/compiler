use crate::compiler::lib::private::std::types::bool_lang::BoolLang;
use crate::compiler::lib::private::std::types::char_lang::CharLang;
use crate::compiler::lib::private::std::types::float::float32_lang::Float32Lang;
use crate::compiler::lib::private::std::types::float::float64_lang::Float64Lang;
use crate::compiler::lib::private::std::types::int::int16_lang::Int16Lang;
use crate::compiler::lib::private::std::types::int::int32_lang::Int32Lang;
use crate::compiler::lib::private::std::types::int::int64_lang::Int64Lang;
use crate::compiler::lib::private::std::types::int::int8_lang::Int8Lang;
use crate::compiler::lib::private::std::types::string_lang::StringLang;
use crate::compiler::lib::private::std::types::uint::uint16_lang::IntU16Lang;
use crate::compiler::lib::private::std::types::uint::uint32_lang::IntU32Lang;
use crate::compiler::lib::private::std::types::uint::uint64_lang::IntU64Lang;
use crate::compiler::lib::private::std::types::uint::uint8_lang::IntU8Lang;

pub enum LangType{
    Bool(bool),
    Char(char),
    String(String),
    Class,
    Enum,
    None,
    Struct,
    Float32(f64),
    Float64(f32),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
}


impl LangType{
    fn convert_type_to_struct(type_lang: LangType) -> Box<dyn std::any::Any>{
        match type_lang {
            LangType::Bool(val) => Box::new(BoolLang::new(val)),
            LangType::Char(val) => Box::new(CharLang::new(val)),
            LangType::String(val) => Box::new(StringLang::new(val)),
            LangType::Float32(val) => Box::new(Float64Lang::new(val)),
            LangType::Float64(val) => Box::new(Float32Lang::new(val)),
            LangType::Int8(val) => Box::new(Int8Lang::new(val)),
            LangType::Int16(val) => Box::new(Int16Lang::new(val)),
            LangType::Int32(val) => Box::new(Int32Lang::new(val)),
            LangType::Int64(val) => Box::new(Int64Lang::new(val)),
            LangType::UInt8(val) => Box::new(IntU8Lang::new(val)),
            LangType::UInt16(val) => Box::new(IntU16Lang::new(val)),
            LangType::UInt32(val) => Box::new(IntU32Lang::new(val)),
            LangType::UInt64(val) => Box::new(IntU64Lang::new(val)),
            _ => {
                Box::new(None)
            }
        }
    }
}