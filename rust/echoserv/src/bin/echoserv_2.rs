/// Simple echo server using `tiny_http` crate
use std::io::prelude::*;
use tiny_http::{Header, Response, Server};

fn main() {
    let addr = "127.0.0.1:18888";

    let server = Server::http(addr).expect("Failed to initiate server");
    for req in server.incoming_requests() {
        println!(
            "{:?} {:?} {:?}",
            req.http_version(),
            req.method(),
            req.url()
        );
        for header in req.headers() {
            println!("{:20} {}", header.field, header.value);
        }

        let content_type = "Content-Type: text/html".parse::<Header>().unwrap();

        let res = Response::from_string(format!(
            "<html><body>Hello. <a href=\"{}\">link</a></body></html>",
            addr
        ))
        .with_status_code(200)
        .with_header(content_type);
        req.respond(res).unwrap();
    }
}
