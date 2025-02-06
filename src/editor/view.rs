use std::{fs::File, io::{self, BufRead, BufReader, IoSlice, IoSliceMut, Read}, path::Path};

use termion::input::TermRead;


pub struct View {
    text: Vec<String>,
    // file: File,
}


impl View {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        // Открываем файл
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        // Считываем строки в вектор
        let mut text = Vec::new();
        for line in reader.lines() {
            text.push(line?); // Обрабатываем ошибку при чтении строки
        }

        // Возвращаем новый экземпляр структуры
        Ok(Self { text })
    }

    // Метод для получения текста
    pub fn get_text(&self) -> &Vec<String> {
        &self.text
    }
}