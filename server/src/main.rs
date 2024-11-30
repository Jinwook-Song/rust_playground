use http::request::Request;
use server::Server;

fn main() {
    let addr = String::from("127.0.0.1:8080");
    let server = Server::new(addr);
    server.run();
}

mod server {

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
}

mod http {
    pub mod request {
        use super::method::Method;
        pub struct Request {
            path: String,
            query_string: Option<String>,
            method: Method,
        }
    }
    pub mod method {
        pub enum Method {
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
    }
}
