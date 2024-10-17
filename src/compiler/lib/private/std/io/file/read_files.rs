<<<<<<< HEAD
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_file_to_string(file_path: &str) -> Result<String, io::Error> {
    let mut result_line = String::new();
    let file = File::open(file_path)?; // Используем ? для обработки ошибок
    let buffer = BufReader::new(file);

    // Чтение файла построчно
    for line in buffer.lines() {
        // Обрабатываем каждую строку
        let line = line?; // Используем ? для обработки ошибок
        result_line.push_str(&line); // Добавляем строку в результат
        result_line.push('\n'); // Добавляем новую строку
    }

    Ok(result_line) // Возвращаем результат
=======
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
>>>>>>> 0b75cfab27fcd10ead4c1faef6dc94aa15a84199
}