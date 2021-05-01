use std::collections::HashMap;
use crate::http::Method;

pub struct Request {
    line: RequestLine,
    head: Head,
    body: Option<String>,
}

impl Request {

    pub fn get(url: &str) -> Self {
        
    }

    pub fn post(url: &str, body: &str) -> Self {

    }

    pub fn new(url: &str, method: Method, head: Head, body: Option<&str>) -> Self {
        let url = URL::parse(url);
        Self {
            line: RequestLine::new(method, )
        }

    }

    fn build(&self) -> String {
        
    }


    fn build_head(&self) -> String {
    }

    fn build_body(&self) -> String {

    }
    
}

struct RequestLine {
    method: Method,
    path: String,
    protocol: String,
}

impl RequestLine {
    
    pub fn new(method: Method, path: &str, protocol: &str) -> Self {
        Self {
            method,
            path: path.to_string(),
            protocol: protocol.to_string(),
        }
    }

    pub fn build(&self) -> String {
        let method = match self.method {
            Method::GET => "GET",
            Method::POST => "POST",
        };

        format!("{} {} {}\n", method, self.path, self.protocol)
    }
}


pub struct Head(HashMap<String, String>);

impl Head {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add(&mut self, key: &str, val: &str) {
        self.0.insert(key.to_string(), val.to_string());
    }

    pub fn remove(&mut self, key: &str) {
        self.0.remove(key);
    }
}


// URL = (scheme "://")? host (":" port)? (path)?
pub struct URL {
    scheme: String,
    host: String,
    port: String,
    path: String,
}

impl URL {
    parse(url: &str) -> Self {
        
    }
}












