use crate::compiler::lib::private::std::types::type_compress::LangType;

pub struct Argument{
    pub name: String,
    pub arg_type: LangType
}

impl Argument {
    fn new(name: String, arg_type: LangType) -> Argument {
        Argument{
            name,
            arg_type
        }
    }

}