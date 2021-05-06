
use std::net;
use std::io::prelude::*;
use std::net::{TcpStream, ToSocketAddrs};
use crate::http::{request::Request, response::Response, url::URL};

pub struct Client {
}


impl Client {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(url: &str) -> Response {

        let parsed_url = URL::parse(url);
        let ip = format!("{}:{}", parsed_url.host(), parsed_url.port())
            .to_socket_addrs()
            .unwrap()
            .nth(0)
            .unwrap();

        let mut req = Request::get(url);
        let req = req.build();

        let mut stream = TcpStream::connect(ip).unwrap();
        stream.write_all(req.as_bytes()).unwrap();

        let mut res = String::new();
        stream.read_to_string(&mut res).unwrap();

        stream.shutdown(net::Shutdown::Both).unwrap();

        let res = Response::parse(&res);

        res
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
        Client::get("example.com");
    }
}





