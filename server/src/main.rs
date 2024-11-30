fn main() {
    let addr = String::from("127.0.0.1:8080");
    let server = Server::new(addr);
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    // Static Method (Associated Function)
    fn new(addr: String) -> Self {
        Self { addr }
    }

    // Instance Method
    fn run(self) {
        println!("✅ Listen on {}", self.addr)
    }
}
