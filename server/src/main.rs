use http::Method;
use http::Request;
use server::Server;

mod http;
mod server;

fn main() {
    // let get = Method::GET;

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();

    // Syntactic sugar of 'while true'
    // 'outer: loop {
    //     'inner: loop {
    //         break 'outer;
    //     }
    // }
}

