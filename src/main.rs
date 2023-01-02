use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

enum MediaType {
    PLAIN,
    HTML,
    CSS,
    JAVASCRIPT,
    JSON,
    PNG,
    JPEG
}

impl MediaType {
    fn parse(extension: &str) -> MediaType {
        match extension {
            "htm" | "html"  => MediaType::HTML,
            "css"           => MediaType::CSS,
            "js"            => MediaType::JAVASCRIPT,
            "json"          => MediaType::JSON,
            "png"           => MediaType::PNG,
            "jpg" | "jpeg"  => MediaType::JPEG,
            "txt" | _       => MediaType::PLAIN
        }
    }

    fn content_type(&self) -> &str {
        match *self {
            MediaType::HTML         => "text/html",
            MediaType::CSS          => "text/css",
            MediaType::JAVASCRIPT   => "application/css",
            MediaType::JSON         => "application/json",
            MediaType::PNG          => "image/png",
            MediaType::JPEG         => "image/jped",
            MediaType::PLAIN        => "text/plain",
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap(); //Use match instead of unwrap later on

        handle_connection(stream);
    }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
    
        println!("Request: {:#?}", http_request);

        const HTTP_VER: &str = "HTTP/1.1";
        const BASE_PATH: &str = "./static";

        let file_path = http_request[0].split(' ').nth(1).unwrap(); //Only works for correct requests
        let extension = file_path.split('.').last().unwrap();
        let media: MediaType = MediaType::parse(&extension);
        let content_type = media.content_type();


        let file_result = fs::File::open(format!("{BASE_PATH}{file_path}"));
        let response;
        let mut response_bytes;
        match file_result {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut buffer = Vec::new();

                reader.read_to_end(&mut buffer).expect("Error when reading file!");

                let len = buffer.len();
                response = format!("{HTTP_VER} 200 OK\r\nContent-Type: {content_type}\r\nContent-Length: {len}\r\n\r\n");
                response_bytes = response.into_bytes();
                response_bytes.append(&mut buffer);

                stream.write_all(&response_bytes).unwrap();
            },
            Err(_error) => {
                response = format!("{HTTP_VER} 404 Not Found\r\n");
                stream.write_all(response.as_bytes()).unwrap();
            }
        }
    }
}
