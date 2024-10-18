use std::io::Error;
use crate::compiler::lexer::lexer::tokenize;
use crate::compiler::lib::private::std::io::file::read_files::read_file_to_string;

mod compiler;



fn main() {
    match read_file_to_string("test/lang/example/file.txt") {
        Ok(result) => {
            let head = tokenize(result);
            for heads in head{
                println!("{:?}", heads)
            }
        }
        Err(result) => {
            eprintln!("{result}",)
        }
    }
}
