use std::collections::HashMap;
use crate::http::Method;
use crate::http::url::URL;

// Example HTTP Request
// ```
// GET / HTTP/1.1
// Host: example.com
// User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:88.0) Gecko/20100101 Firefox/88.0
// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8
// Accept-Language: en-US,en;q=0.5
// Accept-Encoding: gzip, deflate
// Connection: keep-alive
// Upgrade-Insecure-Requests: 1
// If-Modified-Since: Thu, 17 Oct 2019 07:18:26 GMT
// If-None-Match: "3147526947"
// Cache-Control: max-age=0
// ```

pub struct Request {
    line: RequestLine,
    head: Head,
    body: String,
}

impl Request {

    pub fn get(url: &str) -> Self {
        let url = URL::parse(url);
        Self::new(
            &url,
            Method::GET,
            Head::new(&url.host()),
            None,
        )
    }

    pub fn read_host(&self) -> String {
        let head = &self.head;
        let host = match head.0.get("Host") {
            Some(s) => s.clone(),
            None => panic!("No valid Host"),
        };
        host
    }

    // pub fn post(url: &str, body: &str) -> Self {
    //
    // }

    pub fn new(url: &URL, method: Method, head: Head, body: Option<String>) -> Self {
        Self {
            line: RequestLine::new(method, &url.path(), &url.scheme()),
            head,
            body: match body {
                Some(s) => s,
                None => "".to_string(),
            },
        }

    }

    pub fn build(&mut self) -> String {
        format!("{}{}\r\n{}", self.line.build(), self.head.build(), self.body)
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

        format!("{} {} {}\r\n", method, self.path, "HTTP/1.0")
    }
}


pub struct Head(HashMap<String, String>);

impl Head {
    pub fn new(host: &str) -> Self {
        let mut head = Self(HashMap::new());
        head.add("Host", host);
        head
    }

    pub fn add(&mut self, key: &str, val: &str) {
        self.0.insert(key.to_string(), val.to_string());
    }

    pub fn remove(&mut self, key: &str) {
        self.0.remove(key);
    }

    pub fn build(&mut self) -> String {
        let mut result = format!("Host: {}\r\n", self.0["Host"]);
        self.remove("Host");
        for (k, v) in self.0.drain() {
            result = format!("{}{}:{}\r\n", result, k, v);
        }
        result
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let url = "example.com";

        let req = Request::get(url);

        assert_eq!(req.line.path, "/");
        assert_eq!(req.line.protocol, "http");
        assert_eq!(req.head.0.get("Host"), Some(&"example.com".to_string()));
        assert_eq!(req.body, String::new());
    }
    
    #[test]
    fn test_build() {
        let mut req = Request::get("example.com");
        let raw_req = req.build();

        println!("{}", raw_req);
    }

}











