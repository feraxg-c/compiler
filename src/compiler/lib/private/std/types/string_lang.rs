<<<<<<< HEAD
=======
<<<<<<< HEAD
>>>>>>> 9c061fd5b3ab1734c3f3d1c060b92ee7df528ba4

struct StringLang {
    val: String,
    len: usize,
}

impl StringLang {
    fn new(val: String) -> StringLang {
        StringLang {
            val: val.clone(), // Клонируем строку
            len: val.len(),   // Длина строки в usize
        }
    }

    fn convert_type_to_c(&self) -> String {
        format!("char[{}]", self.len)
    }
}
<<<<<<< HEAD
=======
=======

struct StringLang {
    val: String,
    len: usize,
}

impl StringLang {
    fn new(val: String) -> StringLang {
        StringLang {
            val: val.clone(), // Клонируем строку
            len: val.len(),   // Длина строки в usize
        }
    }

    fn convert_type_to_c(&self) -> String {
        format!("char[{}]", self.len)
    }
}
>>>>>>> 0b75cfab27fcd10ead4c1faef6dc94aa15a84199
>>>>>>> 9c061fd5b3ab1734c3f3d1c060b92ee7df528ba4
