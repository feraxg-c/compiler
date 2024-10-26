pub struct LangUInt8{
    pub value: i8
}
pub struct LangUInt16{
    pub value: i16
}
pub struct LangUInt32{
    pub value: i32
}
pub struct LangUInt64{
    pub value: i64
}


impl LangUInt8{
    pub fn new(value: i8) -> LangUInt8 {
        LangUInt8{
            value
        }
    }
}
impl LangUInt16{
    pub fn new(value: i16) -> LangUInt16 {
        LangUInt16{
            value
        }
    }
}
impl LangUInt32{
    pub fn new(value: i32) -> LangUInt32 {
        LangUInt32{
            value
        }
    }
}
impl LangUInt64{
    pub fn new(value: i64) -> LangUInt64 {
        LangUInt64{
            value
        }
    }
}