use std::io::Read;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::net::TcpListener;
use crate::http::Request;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self {
            address
        }
    }

    pub fn run(self) {
        println!("Listening to {}", self.address);
        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, address)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(request) => {
                            println!("Received: {}", String::from_utf8_lossy(&buffer));
                            Request::try_from(&buffer[..]);
                        }
                        Err(e) => {
                            println!("Fail: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Fail: {}", e);
                    continue;
                }
            }
        }
    }
}
