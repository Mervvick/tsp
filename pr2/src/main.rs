use regex::Regex;
use std::{fs, sync::Arc, sync::Mutex, thread};

struct FileAnalysis {
    name: String,
    word_count: usize,
    char_count: usize,
}

struct Task {
    file_analysis: FileAnalysis,
}

fn main() {
    let all_word_count = Arc::new(Mutex::new(0));
    let all_char_count = Arc::new(Mutex::new(0));

    let tasks = vec![
        Task {
            file_analysis: FileAnalysis {
                name: "file1.txt".to_string(),
                word_count: 0,
                char_count: 0,
            },
        },
        Task {
            file_analysis: FileAnalysis {
                name: "file2.txt".to_string(),
                word_count: 0,
                char_count: 0,
            },
        },
        Task {
            file_analysis: FileAnalysis {
                name: "file3.txt".to_string(),
                word_count: 0,
                char_count: 0,
            },
        },
    ];

    thread::scope(|scope| {
        for mut task in tasks {
            let all_word_count_clone = Arc::clone(&all_word_count);
            let all_char_count_clone = Arc::clone(&all_char_count);

            scope.spawn(move || {
                if !fs::exists(&task.file_analysis.name).unwrap() {
                    println!("File '{}' doesn't exists.", &task.file_analysis.name);
                    return;
                }

                let contents = fs::read_to_string(&task.file_analysis.name).expect(
                    "Can't open file. Please, check that file exsits and available to open.",
                );

                task.file_analysis.char_count = contents.len();

                let re = Regex::new("[^a-zA-Z']").unwrap();

                let mut word_vec: Vec<&str> = re.split(&contents).collect();
                word_vec.retain(|&s| s != "");

                task.file_analysis.word_count = word_vec.len();

                println!(
                    "{}: {} words, {} characters.",
                    task.file_analysis.name,
                    task.file_analysis.word_count,
                    task.file_analysis.char_count
                );

                let mut word_count = all_word_count_clone.lock().unwrap();
                *word_count += task.file_analysis.word_count;

                let mut char_count = all_char_count_clone.lock().unwrap();
                *char_count += task.file_analysis.char_count;
            });
        }
    });

    println!(
        "Summary: {} words, {} characters.",
        all_word_count.lock().unwrap(),
        all_char_count.lock().unwrap()
    );
}
