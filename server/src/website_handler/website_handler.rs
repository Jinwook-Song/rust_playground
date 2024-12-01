use super::super::http::{Request, Response, StatusCode};
use super::super::server::Handler;

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, _: &Request) -> Response {
        Response::new(StatusCode::Ok, Some("<h1>TEST</h1>".to_string()))
    }
}
