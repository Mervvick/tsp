# Задание

Необходимо разработать сетевое приложение на языке Rust, которое реализует клиент-серверную архитектуру. Сервер принимает текстовые файлы от клиентов, анализирует их содержимое (количество слов, символов и строк), сохраняет результаты анализа в локальную файловую систему и отправляет ответ клиенту.
# Цели

- Изучение основ сетевого программирования в Rust.
- Практика работы с файловой системой.
- Закрепление навыков обработки текста и работы с многозадачностью.

# Задачи

1. Разработка серверной части:
	- Создать сервер на основе библиотеки tokio или std::net.
	- Реализовать возможность получения текстовых файлов от нескольких клиентов одновременно.
	- Сохранить файлы в локальной файловой системе с уникальными именами.
2. Анализ содержимого файлов:
	- Реализовать подсчет количества строк, слов и символов в каждом полученном файле.
	- Сохранить результаты анализа в текстовый файл (например, analysis_result.txt).
3. Отправка результатов клиенту:
	- Сервер отправляет клиенту анализ содержимого файлов в формате: 
	- Имя файла: file1.txt  
	- Строк: 10, Слов: 50, Символов: 300  
4. Разработка клиентской части:
    - Клиент подключается к серверу, отправляет текстовые файлы, получает и отображает результаты анализа.
5. Обработка ошибок:
    - Сервер должен корректно обрабатывать недоступность файлов, некорректные запросы или другие возможные ошибки.
# Решение

## `client.rs`

```rust
use std::error::Error;
use std::{env, fs, io};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let server_addr = "127.0.0.1:3000";
    let stream: TcpStream = TcpStream::connect(server_addr).await.unwrap();

    send_request(stream, &args[1]).await.unwrap();
}

async fn send_request(stream: TcpStream, filename: &String) -> Result<(), Box<dyn Error>> {
    loop {
        stream.writable().await?;

        if !fs::exists(&filename).unwrap() {
            println!("File '{}' doesn't exists.", &filename);
            return Err("File doesn't exists".into());
        }

        let contents = fs::read_to_string(&filename)
            .expect("Can't open file. Please, check that file exsits and available to open.");

        let mut dat = vec![filename.len() as u8];
        dat.append(&mut filename.clone().into_bytes());
        dat.append(&mut contents.into_bytes());

        match stream.try_write(&dat) {
            Ok(_n) => {
                break;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    let mut buf: [u8; 4096];
    loop {
        stream.readable().await?;
        buf = [0; 4096];
        match stream.try_read(&mut buf) {
            Ok(n) => {
                let mut vec = Vec::with_capacity(n);
                buf.take(n as u64).read_to_end(&mut vec).await?;
                let s = String::from_utf8(buf.to_vec()).unwrap();
                println!("{}", s);
                break;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => return Err(e.into()),
        }
    }

    Ok(())
}
```

## `server.rs`

```rust
use regex::Regex;
use std::io::Write;
use std::{fs::File, str};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 2048];

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                let filename_size = buf[0] as usize;
                let filename = str::from_utf8(&buf[1..1 + filename_size]).unwrap();
                let file_content = str::from_utf8(&buf[filename_size + 1..n]).unwrap();

                File::create("saved_".to_string() + filename)
                    .unwrap()
                    .write(file_content.as_bytes())
                    .unwrap();

                let char_count = file_content.len();

                let re = Regex::new("[^a-zA-Z']").unwrap();

                let mut word_vec: Vec<&str> = re.split(&file_content).collect();
                word_vec.retain(|&s| s != "");

                let word_count = word_vec.len();
                let line_count = file_content.chars().filter(|&c| c == '\n').count() + 1;

                if let Err(e) = socket
                    .write_all(
                        format!(
                            "Filename: {}. Lines: {}, words: {}, characters: {}\n",
                            filename, line_count, word_count, char_count
                        )
                        .as_bytes(),
                    )
                    .await
                {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
```

# Проверка

1. Клиент отправляет серверу файл file1.txt.
2. Сервер сохраняет файл, анализирует его содержимое и отправляет ответ клиенту.
3. Клиент отображает результаты анализа


![alt text](Скрины/Pasted%20image%2020250613205623.png)


## `file1.txt`

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

## `file2.txt`

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