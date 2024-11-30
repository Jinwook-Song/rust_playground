fn main() {
    let get = Method::GET;
    let post = Method::POST;
    let put = Method::PUT;
    let delete = Method::DELETE;

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

struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    TRACE,
    OPTIONS,
    CONNECT,
    PATCH,
}
