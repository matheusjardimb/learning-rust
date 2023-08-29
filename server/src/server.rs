use std::net::TcpListener;

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
                Ok((stream, address)) => {
                    println!("ok")
                }
                Err(e) => {
                    println!("Fail: {}", e);
                    continue;
                }
            }
        }
    }
}
