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

    let mut buf: [u8; 8192];
    loop {
        stream.readable().await?;
        buf = [0; 8192];
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
