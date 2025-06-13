use std::env;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_found() {
        let result = process_file("file1.txt", "one");
        assert_eq!(result, 3);

        let result = process_file("file1.txt", "Snow");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_not_found() {
        let result = process_file("file1.txt", "file");
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
