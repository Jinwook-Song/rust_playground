use std::fs;

use super::super::http::{Method, Request, Response, StatusCode};
use super::super::server::Handler;

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => {
                    let html = fs::read_to_string("docs/index.html")
                        .unwrap_or_else(|_| {
                            "<h1>Failed to load page</h1>".to_string()
                        });
                    Response::new(StatusCode::Ok, Some(html))
                }
                "/about" => Response::new(
                    StatusCode::Ok,
                    Some("About the website".to_string()),
                ),
                _ => Response::new(StatusCode::NotFound, None),
            },

            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
