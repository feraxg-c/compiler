
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
