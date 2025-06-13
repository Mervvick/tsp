# Задание

Необходимо разработать многопоточное консольное приложение на языке Rust, которое анализирует содержимое нескольких текстовых файлов, подсчитывая количество слов и символов в каждом файле. Приложение должно использовать структуры данных, потоки (threads), асинхронное программирование и мьютексы для синхронизации данных.

# Цели

- Практика работы со структурами данных (struct, enum).
- Изучение многопоточности и синхронизации потоков.
- Работа с асинхронным программированием с использованием async/await.

# Задачи

1. Создание структуры данных для хранения результатов анализа:
    -  Разработать структуру FileAnalysis, которая хранит имя файла, количество слов и количество символов.
2. Реализация многопоточной обработки файлов:
	- Создать пул потоков (thread pool) для обработки нескольких файлов одновременно.
    - Использовать мьютексы (Mutex) или атомарные переменные для синхронизации доступа к разделяемым данным.
3. Асинхронная обработка файлов:
    - Использовать async/await для чтения содержимого файлов.
    - Обработать ошибки чтения файлов (например, файл не найден, недоступен).
	- Данные должны обрабатываться и считываться.
4. Вывод результатов в консоль:
    - Отобразить список файлов с количеством слов и символов для каждого.
    - Добавить общий итог для всех обработанных файлов.

# Решение

```rust
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
```


# Проверка

Запуск обработки 3 файлов

![alt text](Скрины/Pasted%20image%2020250613203501.png)

file1.txt
```
Say your prayers, little one, don't forget, my son
To include everyone
I tuck you in, warm within, keep you free from sin
'Til the Sandman, he comes
Sleep with one eye open
Gripping your pillow tight
Exit light
Enter night
Take my hand
We're off to never-never land
Something's wrong, shut the light, heavy thoughts tonight
And they aren't of Snow White
Dreams of war, dreams of liars, dreams of dragons' fire
And of things that will bite, yeah
Sleep with one eye open
Gripping your pillow tight
Exit light
Enter night
Take my hand
We're off to never-never land
Yeah, yeah
Now I lay me down to sleep (now I lay me down to sleep)
Pray the Lord my soul to keep (pray the Lord my soul to keep)
If I die before I wake (if I die before I wake)
Pray the Lord my soul to take (pray the Lord my soul to take)
Hush, little baby, don't say a word
And never mind that noise you heard
It's just the beast under your bed
In your closet, in your head
Exit light
Enter night
Grain of sand
Exit light
Enter night
Take my hand
We're off to never-never land (yeah)
Oh, yeah, yeah, no
We're off to never-never land
Take my hand
We're off to never-never land
Take my hand
We're off to never-never land
We're off to never-never land
We're off to never-never land
We're off to never-never land
```
file2.txt
```
Dog goes woof, cat goes meow
Bird goes tweet, and mouse goes squeak
Cow goes moo. Frog goes croak, and the elephant goes toot
Ducks say quack and fish go blub, and the seal goes ow ow ow
But there's one sound that no one knows...
What does the fox say?
Ring-ding-ding-ding-dingeringeding!
Gering-ding-ding-ding-dingeringeding!
Gering-ding-ding-ding-dingeringeding!
What the fox say?
Wa-pa-pa-pa-pa-pa-pow!
Wa-pa-pa-pa-pa-pa-pow!
Wa-pa-pa-pa-pa-pa-pow!
What the fox say?
Hatee-hatee-hatee-ho!
Hatee-hatee-hatee-ho!
Hatee-hatee-hatee-ho!
What the fox say?
Joff-tchoff-tchoff-tchoffo-tchoffo-tchoff!
Joff-tchoff-tchoff-tchoffo-tchoffo-tchoff!
Joff-tchoff-tchoff-tchoffo-tchoffo-tchoff!
What the fox say?
Big blue eyes, pointy nose, chasing mice, and digging holes
Tiny paws, up the hill, suddenly you're standing still
Your fur is red, so beautiful, like an angel in disguise
But if you meet a friendly horse, will you communicate by mo-o-o-o-orse, mo-o-o-o-orse, mo-o-o-o-orse?
How will you speak to that h-o-o-orse, h-o-o-orse, h-o-o-orse?
What does the fox say?!
Jacha-chacha-chacha-chow!
Jacha-chacha-chacha-chow!
Jacha-chacha-chacha-chow!
What the fox say?
Fraka-kaka-kaka-kaka-kow!
Fraka-kaka-kaka-kaka-kow!
Fraka-kaka-kaka-kaka-kow!
What the fox say?
A-hee-ahee ha-hee!
A-hee-ahee ha-hee!
A-hee-ahee ha-hee!
What the fox say?
A-oo-oo-oo-ooo!
Woo-oo-oo-ooo!
What does the fox say?!
The secret of the fox, ancient mystery
Somewhere deep in the woods, I know you're hiding
What is your sound? Will we ever know?
Will always be a mystery what do you say?
You're my guardian angel hiding in the woods
What is your sound?
A-bubu-duh-bubu-dwee-dum a-bubu-duh-bubu-dwee-dum
Will we ever know?
A-bubu-duh-bubu-dwee-dum
I want to, I want to, I want to know!
A-bubu-duh-bubu-dwee-dum
Bay-buh-day bum-bum bay-dum
```
file3.txt
```
He deals the cards as a meditation
And those he plays never suspect
He doesn't play for the money he wins
He don't play for respect
He deals the cards to find the answer
The sacred geometry of chance
The hidden law of a probable outcome
The numbers lead a dance
I know that the spades are the swords of a soldier
I know that the clubs are weapons of war
I know that diamonds mean money for this art
But that's not the shape of my heart
He may play the Jack of diamonds
He may lay the Queen of spades
He may conceal a King in his hand
While the memory of it fades
I know that the spades are the swords of a soldier
I know that the clubs are weapons of war
I know that diamonds mean money for this art
But that's not the shape of my heart
That's not the shape, the shape of my heart
And if I told you that I loved you
You'd maybe think there's something wrong
I'm not a man of too many faces
The mask I wear is one
But those who speak know nothing
And find out to their cost
Like those who curse their luck in too many places
And those who fear a loss
I know that the spades are the swords of a soldier
I know that the clubs are weapons of war
I know that diamonds mean money for this art
But that's not the shape of my heart
That's not the shape of my heart
That's not the shape, the shape of my heart
```

