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
        println!("✅ Listen on {}", self.addr)
    }
}
