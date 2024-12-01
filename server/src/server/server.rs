use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

use crate::http::{ParseError, Request, Response, StatusCode};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    // Default implementation
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    // Static Method (Associated Function)
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    // Instance Method
    pub fn run(self, mut handler: impl Handler) {
        println!("✅ Listen on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap_or_else(|e| {
            panic!("Failed to bind to address {}: {}", self.addr, e)
        });

        loop {
            match listener.accept() {
                Ok((stream, addr)) => {
                    println!("🔗 Accepted connection from {}", addr);
                    handle_accept(stream, addr, &mut handler);
                }
                Err(e) => println!("❌ Failed to accept connection: {}", e),
            }
        }
    }
}

/// Handles a single client connection.
fn handle_accept<H: Handler>(
    mut stream: TcpStream,
    _: SocketAddr,
    handler: &mut H,
) {
    // Initialize a buffer for reading data
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(_) => {
            println!(
                "Received a request: {}",
                String::from_utf8_lossy(&buffer)
            );

            let response = match Request::try_from(&buffer[..]) {
                Ok(request) => handler.handle_request(&request),
                Err(e) => handler.handle_bad_request(&e),
            };

            if let Err(e) = response.send(&mut stream) {
                println!("Failed to send response: {}", e);
            }
        }
        Err(e) => {
            println!("Failed to read from connection: {}", e)
        }
    }
}
