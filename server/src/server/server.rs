use std::net::TcpListener;

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
                Ok((stream, addr)) => todo!(),
                Err(e) => println!("Failed to connection: {}", e),
            }
        }
    }
}
