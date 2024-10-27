#[derive(Clone)]
pub struct LangFloat32{
    pub value: f32
}

#[derive(Clone)]
pub struct LangFloat64{
    pub value: f64
}

impl LangFloat32{
    pub fn new(value: f32) -> LangFloat32{
        LangFloat32{
            value
        }
    }
}

impl LangFloat64{
    pub fn new(value: f64) -> LangFloat64{
        LangFloat64{
            value
        }
    }
}