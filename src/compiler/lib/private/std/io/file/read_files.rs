use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_to_string(file_path: &str){
    let mut result_line = String::new();
    let file = File::open(file_path);
    let buffer = BufReader::new(file);

    // чтение файла построчно
    for line in buffer.lines() {
        // обрабатываем каждую строку
        let line = line?;
        result_line.push_str(&*line);
        result_line.push('\n')
    }
}