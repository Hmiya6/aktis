use std::net;
use std::io::prelude::*;
use std::net::{TcpStream, ToSocketAddrs};
use crate::http::{request::Request, response::Response, url::URL};
use std::fmt;
use std::error::Error;

// ERROR HANDLING ---------------------
#[derive(Debug)]
pub enum ClientError {
    
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            _ => write!(f, "Undefined error"),
        }
    }
}

impl Error for ClientError {
}
// ------------------------------------

pub struct Client {
}


impl Client {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(url: &str) -> Result<Response, Box<dyn Error>> {
        
        // get IP address using OS's DNS resolver
        let parsed_url = URL::parse(url)?;
        let ip = format!("{}:{}", parsed_url.host(), parsed_url.port()) // create valid URL to find IP address
            .to_socket_addrs()? // get IP addresses from resolver
            .nth(0) // get IPv4 address
            .unwrap();
        
        // create GET request from URL
        let mut req = Request::get(url)?;
        let req = req.build()?;
        
        // send request using OS's TCP socket
        let mut stream = TcpStream::connect(ip)?;
        stream.write_all(req.as_bytes())?;
        
        // read response
        let mut res = String::new();
        stream.read_to_string(&mut res)?;
        
        // shutdown TCP connection
        stream.shutdown(net::Shutdown::Both)?;
        
        // parse response
        let res = Response::parse(&res)?;

        Ok(res)
    }

    // pub fn send(request: &Request) -> Response {
    // 
    // }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get() {
        Client::get("example.com").unwrap();
    }
}





