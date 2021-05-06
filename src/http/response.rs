use std::collections::HashMap;
use crate::utils::consumer::Consumer;

// Example HTTP Response Header
// ```
// HTTP/1.1 200 OK
// Content-Encoding: gzip
// Accept-Ranges: bytes
// Age: 542257
// Cache-Control: max-age=604800
// Content-Type: text/html; charset=UTF-8
// Date: Thu, 06 May 2021 05:24:49 GMT
// Etag: "3147526947"
// Expires: Thu, 13 May 2021 05:24:49 GMT
// Last-Modified: Thu, 17 Oct 2019 07:18:26 GMT
// Server: ECS (oxr/8325)
// Vary: Accept-Encoding
// X-Cache: HIT
// Content-Length: 648
//
//
// ```



pub struct Response {
    status_line: StatusLine,
    header: Header,
    body: String,
}

impl Response {
    
    pub fn parse(response: &str) -> Self {
        // println!("RAW------:\n{:?}", response);
        let (first_line, rest) = response.split_once("\r\n").unwrap();
        let status_line = StatusLine::parse(first_line);
        
        let (header_str, rest) = rest.split_once("\r\n\r\n").unwrap();
        let header = Header::parse(header_str);

        Self {
            status_line,
            header,
            body: rest.to_string(),
        }
    }
}

pub struct StatusLine {
    status: String,
    status_code: usize,
    proto: String,
}

impl StatusLine {
    pub fn parse(line: &str) -> Self {

        let mut con = Consumer::new(line);
        
        let proto = match con.next_until_space() {
            Some(s) => {
                con.skip_space();
                s
            },
            None => panic!("Invalid Response: there is no status line"),
        };

        let status_code = match con.to_usize() {
            Some(code) => {
                con.skip_space();
                code
            },
            None => panic!("Invalid Response: there is no status code"),
        };

        let status = match con.next_until_space() {
            Some(s) => {
                con.skip_space();
                s
            },
            None => panic!("Invalid Response: there is no status"),
        };

        Self {
            status,
            status_code,
            proto,
        }
    }
}


pub struct Header(HashMap<String, String>);

impl Header {
    pub fn parse(src: &str) -> Self {
        let mut header = Self(HashMap::new());
        let lines = src.split("\r\n");
        // println!("{:?}", lines.to_owned().collect::<Vec<&str>>());
        for line in lines {
            // println!("{}", line);
            let (key, value) = line.split_once(":").unwrap();
            header.0.insert(key.to_string(), value.trim().to_string());
        }
        header
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_resposne() {
        let raw_res = "HTTP/1.1 200 OK\r
Content-Encoding: gzip\r
Accept-Ranges: bytes\r
Age: 579161\r
Cache-Control: max-age=604800\r
Content-Type: text/html; charset=UTF-8\r
Date: Thu, 06 May 2021 06:39:14 GMT\r
Etag: \"3147526947+ident\"\r
Expires: Thu, 13 May 2021 06:39:14 GMT\r
Last-Modified: Thu, 17 Oct 2019 07:18:26 GMT\r
Server: ECS (sec/96DC)\r
Vary: Accept-Encoding\r
X-Cache: HIT\r
Content-Length: 648\r
\r
body";
        let res = Response::parse(raw_res);
        
        assert_eq!(res.status_line.status, "OK");
        assert_eq!(res.status_line.status_code, 200);
        assert_eq!(res.status_line.proto, "HTTP/1.1");
        assert_eq!(res.header.0.get("Age"), Some(&"579161".to_string()));
        assert_eq!(res.body, "body");
    }
}






