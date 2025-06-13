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
