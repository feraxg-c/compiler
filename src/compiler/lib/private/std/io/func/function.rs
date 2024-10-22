use crate::compiler::lib::private::std::io::arg::argument::Argument;

struct LangFunc{
    name: String,
    arg: Vec<Argument>,
}


impl LangFunc{

    fn new(name: String, arg: Vec<Argument>) -> LangFunc{
        LangFunc{
            name,
            arg
        }
    }

}