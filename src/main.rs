use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;

fn main() {
    println!("Initializing TCP Listener...");

    let listener = TcpListener::bind("0.0.0.0:8080").expect("Cannot bind to Port");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_conn(stream);
    }
}

fn read_file(filename: &str) -> String {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    contents
}

fn http_response(status: &str, filename: &str, mut stream: TcpStream) {
    let html = read_file(filename);

    let response = format!("HTTP/1.1 {}\r\nContent-Type: text/html\r\n\r\n{}", status, html);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_conn(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("{}", String::from_utf8_lossy(&buffer));

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        http_response("200 OK", "./hello.html", stream);
    } else {
        http_response("404 NOT FOUND", "./404.html", stream);
    }
}
