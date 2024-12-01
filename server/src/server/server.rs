use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

use crate::http::{Request, Response, StatusCode};

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
        let listener = TcpListener::bind(&self.addr).unwrap_or_else(|e| {
            panic!("Failed to bind to address {}: {}", self.addr, e)
        });

        loop {
            match listener.accept() {
                Ok((stream, addr)) => {
                    println!("🔗 Accepted connection from {}", addr);
                    handle_accept(stream, addr);
                }
                Err(e) => println!("❌ Failed to accept connection: {}", e),
            }
        }
    }
}

/// Handles a single client connection.
fn handle_accept(mut stream: TcpStream, _: SocketAddr) {
    // Initialize a buffer for reading data
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(_) => {
            println!(
                "Received a request: {}",
                String::from_utf8_lossy(&buffer)
            );

            let response = match Request::try_from(&buffer[..]) {
                Ok(request) => {
                    dbg!(request);
                    Response::new(
                        StatusCode::Ok,
                        Some("<h1>Working</h1>".to_string()),
                    )
                }
                Err(e) => {
                    println!("❌ Failed to parse request: {}", e);
                    Response::new(StatusCode::BadRequest, None)
                }
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
