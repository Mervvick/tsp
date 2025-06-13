# Задание

Необходимо разработать консольное приложение на языке Rust, которое выполняет обработку текстовых файлов с базовыми функциями, такими как открытие, чтение, поиск определенного слова и вывод количества его повторений. Дополнительно реализовать функции работы с памятью с использованием системы владения и заимствования.

# Цели

* Практика работы с базовыми синтаксическими конструкциями языка Rust. 
* Изучение системы владения, заимствования и ссылок. 
* Работа с файловой системой и обработка ввода/вывода.
# Задачи

1. Создать консольное приложение с вводом параметров через командную строку: 
	- Указание пути к текстовому файлу. 
	- Задание слова для поиска. 
2. Реализовать функцию чтения содержимого текстового файла: 
	- Использовать систему владения для передачи данных. 
	- Учитывать обработку ошибок (например, файл не найден). 
3. Реализовать поиск заданного слова в тексте: 
	- Определить количество его повторений. 
	- Использовать неизменяемые ссылки для анализа содержимого. 
4. Вывести результаты в консоль в формате: 
	- Общее количество слов в файле. 
	- Количество повторений заданного слова. 
5. Реализовать базовый тест для проверки функции поиска слова.

# Решение

main.rs
```rust
use std::env;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_found() {
        let result = process_file("file1.txt", "sun");
        assert_eq!(result, 3);

        let result = process_file("file1.txt", "Sun");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_not_found() {
        let result = process_file("file1.txt", "gun");
        assert_eq!(result, 0);
    }
}

fn clean_word(word: &str) -> String {
    word.chars()
        .filter(|c| c.is_alphabetic() || *c == '\'')
        .collect()
}

fn extract_words(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|word| clean_word(word))
        .filter(|word| !word.is_empty())
        .collect()
}

fn count_occurrences(words: &[String], target: &str) -> u32 {
    words.iter()
        .filter(|&word| word == target)
        .count() as u32
}

fn display_results(content: &str, words: &[String], target: &str, count: u32) {
    println!("File contains:\n========================\n{content}\n========================");
    println!(
        "Total word count: {}\nCount of '{}' word: {}",
        words.len(),
        target,
        count
    );
}

fn process_file(filename: &str, str_to_find: &str) -> u32 {
    let file_content = fs::read_to_string(filename)
        .expect("Can't open file. Please, check that file exsits and available to open.");

    let words = extract_words(&file_content);
    let found_count = count_occurrences(&words, str_to_find);
    
    display_results(&file_content, &words, str_to_find, found_count);
    
    found_count
}

fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() != 3 {
        println!("Usage: {} <file> <string>", arguments[0]);
        return;
    }

    process_file(&arguments[1], &arguments[2]);
}
```

# Проверка
## Тесты

### Проверка количества слов "one" (3 штуки) и "Snow" (1)

![alt text](Скрины/Pasted%20image%2020250613202018.png)
![alt text](Скрины/Pasted%20image%2020250613202026.png)


### Проверка наличия слова "file" (0)

![alt text](Скрины/Pasted%20image%2020250613201948.png)

## Использование 

Показ содержимого файла и подсчет количества слов "one"

![alt text](Скрины/Pasted%20image%2020250613201839.png)