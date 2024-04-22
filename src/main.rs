// Uncomment this block to pass the first stage
use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut reader: BufReader<TcpStream>, mut writer: BufWriter<TcpStream>) {
    let mut request_string = String::new();
    // we read_line and not read string 'cause read string read to EOF ... and it will never append ...
    reader.read_line(&mut request_string).unwrap();
    let path = request_string
        .split("\r\n")
        .nth(0)
        .unwrap()
        .split(" ")
        .nth(1)
        .unwrap();

    if path == "/" {
        writer.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
        return;
    }
    writer
        .write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
        .unwrap();
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for rstream in listener.incoming() {
        match rstream {
            Ok(rstream) => {
                let wstream = rstream.try_clone().unwrap();
                let reader: BufReader<TcpStream> = BufReader::new(rstream);
                let writer = BufWriter::new(wstream);
                handle_connection(reader, writer)
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
