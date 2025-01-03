use std::fs::{self, read_to_string};

use super::super::http::{Method, Request, Response, StatusCode};
use super::super::server::Handler;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        // 상대경로 제거, 다른 파일 시스템에 접근 차단
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!(
                        "Directory Traversal Attack Attempted: {}",
                        file_path
                    );
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => {
                    Response::new(StatusCode::Ok, self.read_file("index.html"))
                }
                "/about" => {
                    Response::new(StatusCode::Ok, self.read_file("about.html"))
                }
                path => match self.read_file(path) {
                    Some(contents) => {
                        Response::new(StatusCode::Ok, Some(contents))
                    }
                    None => Response::new(StatusCode::NotFound, None),
                },
            },

            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
