// Uncomment this block to pass the first stage
use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut reader: BufReader<TcpStream>, mut writer: BufWriter<TcpStream>) {
    // Base response is 404 until we handle it
    let mut response = String::from("HTTP/1.1 404 Not Found\r\n\r\n");

    let mut request_lines = Vec::new();

    // Read all request lines until we found an empty line
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => break, // End
            Ok(_) => {
                if line.trim().is_empty() {
                    break; // Empty line, end of request
                }
                request_lines.push(line.trim().to_string());
            }
            Err(err) => {
                eprintln!("Erreur de lecture de la requête : {}", err);
                return;
            }
        }
    }

    let path = request_lines.get(0).unwrap().split(" ").nth(1).unwrap();

    if path == "/" {
        response = String::from("HTTP/1.1 200 OK\r\n\r\n");
    }

    let path_lenght = path.len();

    if path_lenght > 0 {
        let first_path = path.split("/").nth(1).unwrap();

        if path == "/user-agent" {
            let user_agent_line = request_lines.iter().find(|&x| x.contains("User")).unwrap();

            println!("{:?}", user_agent_line);

            let text = user_agent_line.split(": ").last().unwrap();

            println!("{:?}", text);

            let content_lenght: usize = text.len();

            response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n\r\n",
                    content_lenght,
                    text
                );
        } else if path.split("/").count() <= 2 || first_path == "echo" {
            let text = if first_path == "echo" {
                path.replace("/echo/", "")
            } else {
                path.replace("/", "")
            };

            let content_lenght: usize = text.len();

            response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n\r\n",
                    content_lenght,
                    text
                );
        }
    }

    // Send response to client
    writer.write_all(response.as_bytes()).unwrap();
    writer.flush().unwrap();
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let reader = BufReader::new(stream.try_clone().unwrap());
                let writer = BufWriter::new(stream.try_clone().unwrap());
                handle_connection(reader, writer)
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
