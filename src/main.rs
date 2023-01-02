use std::{
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

        let first_line: Vec<&str> = http_request[0].split(' ').collect();
        let extension = first_line[1].split('.').last().unwrap();
        let media: MediaType = MediaType::parse(&extension);
        println!("MIME: {}", media.content_type());
    }
}
