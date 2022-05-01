/// Simple echo server using `tiny_http` crate
use std::io::prelude::*;
use tiny_http::{Response, Server};

fn main() {
    let addr = "0.0.0.0:18888";

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

        let res = Response::from_string("Hello");
        req.respond(res).unwrap();
    }
}
