use std::fs;

use crate::{
    http::{Method, Request, Response, StatusCode},
    server::Handler,
};

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let file_path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(file_path) {
            Ok(path) => {
                if !path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory traversal attack detected: {}", path.display());
                    None
                }
            }
            Err(_) => return None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                path => match self.read_file(path) {
                    Some(content) => Response::new(StatusCode::Ok, Some(content)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
