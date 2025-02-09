use std::{fmt::{self, Error}, fs::File, io::{self, BufRead, BufReader, IoSlice, IoSliceMut, Read, Write}, path::Path};
use termion::input::TermRead;
use std::fs::OpenOptions;

#[derive(Default)]
pub struct View {
    text: Vec<String>,
    file_path: String,
}

impl View {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);

        let mut text = Vec::new();
        for line in reader.lines() {
            text.push(line?);
        }

        let file_path = path.as_ref().to_string_lossy().into_owned();

        Ok(Self { text, file_path })
    }

    // Метод для получения текста
    pub fn get_text(&self) -> &Vec<String> {
        &self.text
    }

    pub fn write_to_file(&self) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)   // Разрешаем запись
            .truncate(true) // Очищаем файл перед записью
            .create(true)  // Создаём файл, если он не существует
            .open(&self.file_path)?; // Открываем файл по пути `file_path`

        for line in &self.text {
            file.write_all(line.as_bytes());
            file.write_all("\r\n".as_bytes());
        }

        Ok(())
    }

    pub fn add_line(&mut self, line: String, index: usize) {
        self.text.insert(index, line);
    }

    pub fn insert_char(&mut self, str_num: usize, char_pos: usize, new_char: char) -> Result<(), String> {
        // Проверяем, что номер строки находится в допустимых пределах
        if str_num >= self.text.len() {
            return Err(format!(
                "Неверный номер строки: {} (доступно {} строк)",
                str_num,
                self.text.len()
            ));
        }

        // Получаем изменяемую ссылку на строку
        let line = &mut self.text[str_num];

        // Проверяем, что позиция символа находится в допустимых пределах
        if char_pos > line.len() {
            return Err(format!(
                "Неверная позиция символа: {} (длина строки: {})",
                char_pos,
                line.len()
            ));
        }

        // Вставляем символ в строку
        line.insert(char_pos, new_char);

        Ok(())
    }
}