use std::collections::HashMap;
use crate::utils::consumer::Consumer;
use std::fmt;
use std::error::Error;

// ERROR HANDLING ------------------------
#[derive(Debug)]
pub enum ResponseError {
    NoLine,
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResponseError::NoLine => write!(f, "Response error: cannot find the first line, invalid HTTP response"),
            _ => write!(f, "Undefined error"),
        }
    }
}

impl Error for ResponseError {}

// ---------------------------------------

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
    
    pub fn parse(response: &str) -> Result<Self, Box<dyn Error>> {
        // println!("RAW------:\n{:?}", response);
        let (first_line, rest) = match response.split_once("\r\n") {
            Some(v) => v,
            None => return Err(Box::new(ResponseError::NoLine)),
        };
        let status_line = StatusLine::parse(first_line)?;
        
        let (header_str, rest) = rest.split_once("\r\n\r\n").unwrap();
        let header = Header::parse(header_str)?;

        Ok(Self {
            status_line,
            header,
            body: rest.to_string(),
        })
    }
}


// ERRROR HANDLING -----------------------
//
// TODO
// - validate status line (status, status code, and protocol)
#[derive(Debug)]
pub enum StatusLineError {
    NoStatus,
    NoStatusCode,
    NoProtocol,
}

impl fmt::Display for StatusLineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NoStatus => write!(f, "Status line error: there is no valid status"),
            NoStatusCode => write!(f, "Status line error: there is no valid status code"),
            NoProtocol => write!(f, "Status line error: there is no valid protocol"),
            _ => write!(f, "Undefined error"),
        }
    }
}

impl Error for StatusLineError {}
// ---------------------------------------

// Example status line:
// ```
// HTTP/1.1 200 OK
// ```

pub struct StatusLine {
    status: String,
    status_code: usize,
    proto: String,
}

impl StatusLine {
    pub fn parse(line: &str) -> Result<Self, StatusLineError> {

        let mut con = Consumer::new(line);
        
        let proto = match con.next_until_space() {
            Some(s) => {
                con.skip_space();
                s
            },
            None => return Err(StatusLineError::NoProtocol),
        };

        let status_code = match con.to_usize() {
            Some(code) => {
                con.skip_space();
                code
            },
            None => return Err(StatusLineError::NoStatusCode),
        };

        let status = match con.next_until_space() {
            Some(s) => {
                con.skip_space();
                s
            },
            None => return Err(StatusLineError::NoStatus),
        };

        Ok(Self {
            status,
            status_code,
            proto,
        })
    }
}

// ERROR HANDING -------------------------
#[derive(Debug)]
pub enum HeaderError {
    InvalidHeader(String),
}

impl fmt::Display for HeaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HeaderError::InvalidHeader(s) => write!(f, "Response header error: found invalid header: `{}`", s),
            _ => write!(f, "Undefined error"),
        }
    }
}

impl Error for HeaderError {}
// ---------------------------------------


pub struct Header(HashMap<String, String>);

impl Header {
    pub fn parse(src: &str) -> Result<Self, HeaderError> {
        let mut header = Self(HashMap::new());
        let lines = src.split("\r\n");

        for line in lines {
            let (key, value) = match line.split_once(":") {
                Some(v) => v,
                None => return Err(HeaderError::InvalidHeader(line.to_string())),
            };
            header.0.insert(key.to_string(), value.trim().to_string());
        }
        Ok(header)
    }
}



// test --------------------------------
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
        let res = Response::parse(raw_res).unwrap();
        
        assert_eq!(res.status_line.status, "OK");
        assert_eq!(res.status_line.status_code, 200);
        assert_eq!(res.status_line.proto, "HTTP/1.1");
        assert_eq!(res.header.0.get("Age"), Some(&"579161".to_string()));
        assert_eq!(res.body, "body");
    }
}






