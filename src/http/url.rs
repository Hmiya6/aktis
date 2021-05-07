use std::error::Error;
use std::fmt;
use crate::utils::consumer::Consumer;

// ERROR HANDLING ----------------------
#[derive(Debug)]
pub enum URLError {
    NoHost,
}

impl fmt::Display for URLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            URLError::NoHost => write!(f, "URL error: there is no host."),
            _ => write!(f, "Undefined error: I don't know why you are here."),
        }
    }
}

impl Error for URLError {}


// ------------------------------------

// URL = (scheme "://")? host (path)? 
pub struct URL {
    scheme: String,
    host: String,
    port: usize,
    path: String,
}

impl URL {
    pub fn parse(url: &str) -> Result<Self, URLError> {
        
        let mut consumer = Consumer::new(url);
        
        let scheme = match consumer.next_until("://") {
            Some(s) => {
                consumer.next_n("://".len());
                s
            },
            None => "http".to_string(),
        };

        let host = match consumer.next_until("/") {
            Some(s1) => s1,
            None => match consumer.next_until_space() {
                Some(s2) => s2,
                None => return Err(URLError::NoHost),
            },
        };

        let path = match consumer.next_until_space() {
            Some(s) => s,
            None => "/".to_string(),
        };

        Ok(Self {
            scheme,
            host,
            port: 80,
            path,
        })
    }

    pub fn scheme(&self) -> String {
        self.scheme.to_owned()
    }

    pub fn host(&self) -> String {
        self.host.to_owned()
    }

    pub fn port(&self) -> usize {
        self.port.to_owned()
    }

    pub fn path(&self) -> String {
        self.path.to_owned()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_url() {
        test_parse("example.com/", "http", "example.com", "/");
        test_parse("http://example.co.jp", "http", "example.co.jp", "/");
        test_parse("https://example.com/test", "https", "example.com", "/test");
    }

    fn test_parse(url: &str, scheme: &str, host: &str, path: &str) {
        println!("{}", url);
        let url = URL::parse(url).unwrap();
        
        assert_eq!(url.scheme, scheme);
        assert_eq!(url.host, host);
        assert_eq!(url.path, path);
    }
}







