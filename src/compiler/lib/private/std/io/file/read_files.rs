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
}