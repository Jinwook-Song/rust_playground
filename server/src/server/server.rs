use std::{io::Read, net::TcpListener};

pub struct Server {
    addr: String,
}

impl Server {
    // Static Method (Associated Function)
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    // Instance Method
    pub fn run(self) {
        println!("✅ Listen on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    // Array: 사용하기전에 초기화
                    let mut buffer = [0; 1024];

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer))
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
