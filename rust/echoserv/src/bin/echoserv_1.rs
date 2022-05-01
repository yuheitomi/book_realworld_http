use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let port = ":18888";
    let listener =
        TcpListener::bind(format!("127.0.0.1{}", port)).expect("Failed to bind connection");
    eprintln!("Listening to... {}", listener.local_addr().unwrap());
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let _size = stream.read(&mut buffer).unwrap();

    let req = String::from_utf8_lossy(&buffer);
    eprintln!("{}", req);

    let response = "HTTP/1.1 200 OK\r\n\r\n<html><body>hello</body></html>";
    let _size = stream
        .write(response.as_bytes())
        .expect("Failed to write to stream");
    stream.flush().expect("Failed to flush");
}
