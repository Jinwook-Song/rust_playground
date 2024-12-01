#![allow(dead_code)]
#![allow(unused_imports)]

mod http;
mod server;
mod website_handler;

use http::Method;
use http::Request;
use server::Server;
use website_handler::WebsiteHandler;

fn main() {
    let addr = String::from("127.0.0.1:8080");
    let server = Server::new(addr);
    server.run(WebsiteHandler);
}
