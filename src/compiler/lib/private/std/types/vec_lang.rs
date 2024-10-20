use rand::Rng; // Импортируем Rng для генерации случайных чисел
use crate::compiler::lib::private::std::types::type_compress::{LangType, TraitTypeFn};

pub struct VecLang {
    pub(crate) val: Vec<LangType>,
}

impl VecLang {
    pub fn new(val: Vec<LangType>) -> VecLang {
        VecLang { val }
    }
}

impl TraitTypeFn for VecLang {
    fn convert_type_to_c(&self) -> String {
        "List".to_string()
    }


    fn create_c_variable(&self, name: String) -> String {
        // cоздаем список
        let mut ret_val = format!(
            "List {} = createList();",
            name
        );

        let mut rng = rand::thread_rng();

        for lang_type in &self.val {
            let random_hash: String = (0..8)
                .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
                .collect();

            let value = lang_type.create_c_variable(random_hash.clone());

            ret_val += &format!("\n{} {} = {};", value, random_hash, value);
            ret_val += &format!("\nappendList({}, {});", name, random_hash);
        }

        ret_val
    }

}