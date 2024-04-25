use std::{
    env,
    fs::{self, File},
    io::{BufReader, BufWriter, Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    thread,
};

fn parse_request(mut reader: BufReader<TcpStream>) -> (String, Vec<String>) {
    let mut buffer = [0; 2048];
    reader.read(&mut buffer).unwrap();
    let request_str = std::str::from_utf8(&buffer).unwrap();

    let lines: Vec<String> = request_str.lines().map(|line| line.to_string()).collect();

    // get body
    let mut collect = false;
    let mut body = String::from("");
    for line in &lines {
        if collect {
            body.push_str(line);
        }
        if line.is_empty() {
            collect = true;
        }
    }
    body = body.trim_matches(char::from(0)).to_string();

    (body, lines)
}

fn handle_connection(
    mut writer: BufWriter<TcpStream>,
    parsed_request: (String, Vec<String>),
) {
    // Create base response variable to be overrited
    let mut response = String::new();

    let request_lines = parsed_request.1;

    let body = parsed_request.0;

    let method = request_lines.get(0).unwrap().split(" ").nth(0).unwrap();
    let path = request_lines.get(0).unwrap().split(" ").nth(1).unwrap();

    let args: Vec<String> = env::args().collect();
    let dir = args.last().unwrap();
    let filename = path.replace("/files", "");

    if method == "POST" {
        let file_path = format!("{}/{}", dir, filename);

        if Path::new(&file_path).exists() {
            response = String::from("HTTP/1.1 404 Not Found\r\n\r\n");
        } else {
            let mut file = File::create(file_path).unwrap();
            file.write_all(body.as_bytes()).unwrap();

            response = String::from("HTTP/1.1 201 Created\r\n\r\n");
        }
    }

    if path == "/" {
        response = String::from("HTTP/1.1 200 OK\r\n\r\n");
    }

    if path.starts_with("/files") && response.len() == 0 {
        if Path::new(&dir).exists() {
            let file_content = fs::read_to_string(format!("{}/{}", dir, filename));

            match file_content {
                Ok(file_content) => {
                    let content_lenght = file_content.len();
                    response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}\r\n\r\n",
                            content_lenght,
                            file_content
                        );
                }
                Err(_) => {
                    response = String::from("HTTP/1.1 404 Not Found\r\n\r\n");
                }
            }
        }
    }

    let path_lenght = path.len();

    if path_lenght > 0 && response.len() == 0 {
        let first_path = path.split("/").nth(1).unwrap();

        if path == "/user-agent" {
            let user_agent_line = request_lines.iter().find(|&x| x.contains("User")).unwrap();

            let text = user_agent_line.split(": ").last().unwrap();

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
    if response.len() == 0 {
        response = String::from("HTTP/1.1 404 Not Found\r\n\r\n");
    }

    // Send response to client
    writer.write_all(response.as_bytes()).unwrap();
    writer.flush().unwrap();
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        thread::spawn(|| match stream {
            Ok(stream) => {
                let reader = BufReader::new(stream.try_clone().unwrap());
                let writer = BufWriter::new(stream.try_clone().unwrap());
                let parsed_request = parse_request(reader);
                handle_connection(writer, parsed_request)
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        });
    }

    Ok(())
}
