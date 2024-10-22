use crate::compiler::lib::private::std::io::arg::argument::Argument;
use crate::compiler::lib::private::std::types::type_compress::{LangType, TraitTypeFn};


pub struct StructLang {
    name: String,
    val: Vec<Argument>,
}

impl StructLang {
    pub fn new(name: String, val: Vec<Argument>) -> StructLang {
        StructLang {
            name,
            val
        }
    }
}

impl TraitTypeFn for StructLang {
    fn convert_type_to_c(&self) -> String {
        self.name.clone()
    }

    fn create_c_variable(&self, var_name: String) -> String {
        let mut args = String::new();
        for arg in &self.val {
            args.push_str(&arg.arg_type.convert_type_to_c());
            args.push(' ');
            args.push_str(&arg.name);
            args.push(';');
            args.push('\n');
        }


        format!(
            "typedef struct {{\n{}\n}} {};",
            args.trim_end(),
            var_name
        )
    }
}