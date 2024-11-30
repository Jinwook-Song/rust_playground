mod http;
mod server;

use http::Method;
use http::Request;
use server::Server;

fn main() {
    let addr = String::from("127.0.0.1:8080");
    let server = Server::new(addr);
    server.run();
}
