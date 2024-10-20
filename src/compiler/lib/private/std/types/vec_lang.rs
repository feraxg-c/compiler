use crate::compiler::lib::private::std::types::type_compress::LangType;

pub struct VecLang {
    pub(crate) val: Vec<LangType>
}

impl VecLang {
    pub fn new(val: Vec<LangType>) -> VecLang {
       VecLang {
            val
        }
    }

    pub fn convert_type_to_c() -> String {
        "List".to_string()
    }

    pub fn create_c_variable(&self, name: String)  {}
}
