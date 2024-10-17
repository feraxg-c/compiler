<<<<<<< HEAD

use crate::compiler::lexer::lexer::tokenize;
use crate::compiler::lib::private::std::io::file::read_files::read_file_to_string;

mod compiler;

fn main() {
    let file_path = "test/lang/file.txt";

    match read_file_to_string(file_path) {
        Ok(content) => {
            println!("Содержимое файла:\n{}", content);

            // Токенизация
            let tokens = tokenize(content);
            println!("Токены: {:?}", tokens);

        },
        Err(e) => eprintln!("Ошибка при чтении файла: {}", e),
    }
}
=======

<<<<<<< HEAD
use crate::compiler::lexer::lexer::tokenize;
use crate::compiler::lib::private::std::io::file::read_files::read_file_to_string;
=======

>>>>>>> 0b75cfab27fcd10ead4c1faef6dc94aa15a84199

mod compiler;

fn main() {
<<<<<<< HEAD
    let file_path = "test/lang/file.txt";

    match read_file_to_string(file_path) {
        Ok(content) => {
            println!("Содержимое файла:\n{}", content);

            // Токенизация
            let tokens = tokenize(content);
            println!("Токены: {:?}", tokens);

        },
        Err(e) => eprintln!("Ошибка при чтении файла: {}", e),
    }
=======


>>>>>>> 0b75cfab27fcd10ead4c1faef6dc94aa15a84199
}
>>>>>>> 9c061fd5b3ab1734c3f3d1c060b92ee7df528ba4
