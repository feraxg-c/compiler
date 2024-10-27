#[derive(Clone)]
pub struct LangInt8{
    pub value: i8
}
#[derive(Clone)]
pub struct LangInt16{
    pub value: i16
}
#[derive(Clone)]
pub struct LangInt32{
    pub value: i32
}
#[derive(Clone)]
pub struct LangInt64{
    pub value: i64
}


impl LangInt8{
    pub fn new(value: i8) -> LangInt8 {
        LangInt8{
            value
        }
    }
}
impl LangInt16{
    pub fn new(value: i16) -> LangInt16 {
        LangInt16{
            value
        }
    }
}
impl LangInt32{
    pub fn new(value: i32) -> LangInt32 {
        LangInt32{
            value
        }
    }
}
impl LangInt64{
    pub fn new(value: i64) -> LangInt64 {
        LangInt64{
            value
        }
    }
}