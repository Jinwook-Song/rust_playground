#![allow(dead_code)]
#![allow(unused_imports)]

mod http;
mod server;
mod website_handler;

use std::env;

use http::Method;
use http::Request;
use server::Server;
use website_handler::WebsiteHandler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let addr = String::from("127.0.0.1:8080");
    let server = Server::new(addr);
    server.run(WebsiteHandler::new(public_path));
}
