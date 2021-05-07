
// `Iter`-like structure.
// It only deals with &str (Vec<char>) (, so that we can build code simply).
pub struct Consumer {
    queue: Vec<char>,
    pos: usize,
}


impl Consumer {
    pub fn new(s: &str) -> Self {
        let vec = s.chars().collect::<Vec<char>>();
        Self {
            queue: vec,
            pos: 0,
        }
    }
    
    // Return next char as `String`.
    pub fn next(&mut self) -> Option<String> {
        if let Some(c) = self.next_char() {
            Some(c.to_string())
        } else {
            None
        }
    }
    
    // Inner function for `next` and `next_n`
    // Return next char if `self.queue` has the next element,
    fn next_char(&mut self) -> Option<char> {
        if self.pos < self.queue.len() {
            let res = self.queue[self.pos];
            self.pos +=1;
            Some(res)
        } else {
            None
        }
    }
    
    // return next n chars as `String`
    pub fn next_n(&mut self, n: usize) -> Option<String> {
        let mut vec = Vec::new();
        for _ in 0..n {
            if let Some(c) = self.next_char() {
                vec.push(c);
            } else {
                return None;
            }
        }
        let res = vec.into_iter().collect::<String>();
        Some(res)

    }
    
    // return string from `self.pos` to next white space
    pub fn next_until_space(&mut self) -> Option<String> {
        let mut vec = Vec::new();

        if self.peek().is_none() {
            return None;
        }

        loop {
            match self.peek_char() {
                Some(c) => {
                    match c {
                        ' ' | '\t' => {
                            break;
                        }
                        _ => {
                            self.next();
                            vec.push(c);
                        }
                    }
                },
                None => {
                    break;
                }
            }
        }
        let res = vec.into_iter().collect::<String>();
        Some(res)
    }
    
    // return a char as `String`
    pub fn peek(&self) -> Option<String> {
        if let Some(c) = self.peek_char() {
            Some(c.to_string())
        } else {
            None
        }

    }
    
    // inner function for mainly `peek` and `peek_n`
    fn peek_char(&self) -> Option<char> {
        let res: char;
        if self.pos < self.queue.len() {
            res = self.queue[self.pos];
            Some(res)
        } else {
            None
        }
    }
    
    // return chars as `String`
    pub fn peek_n(&self, n: usize) -> Option<String> {
        let mut vec = Vec::new();
        for i in 0..n {
            if self.pos+i < self.queue.len() {
                vec.push(self.queue[self.pos+i]);
            } else {
                return None;
            }
        }
        let res = vec.iter().collect::<String>();
        Some(res)
    }
    
    // return `usize` integer
    pub fn to_usize(&mut self) -> Option<usize> {
        let mut result: usize = 0;

        // check whether the first char is number
        match self.peek_char() {
            Some(c) => {
                match c {
                    '0'..='9' => {
                    }
                    _ => {
                        return None;
                    }
                }
            }
            None => {
                return None;
            }
        }

        loop {
            match self.peek_char() {
                Some(c) => {
                    match c {
                        '0'..='9' => {
                            self.next();
                            let n = c.to_digit(10).unwrap() as usize;
                            result = result*10 + n;
                        }
                        _ => {
                            break;
                        }
                    }
                }
                None => {
                    break;
                }
            }
        }
        return Some(result);
    }
    
    // skip white spaces
    pub fn skip_space(&mut self) {
        loop {
            if let Some(c) = self.peek_char() {
                if " \t".contains(c) {
                    self.next_char();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }


    // Example: read URL
    // ```rust
    // let mut consumer = Consumer::new("http://example.com/rust");
    // let protocol = consumer.next_until("://").unwrap(); // == "http"
    // consumer.next_n(3);
    // let host = consumer.next_until("/").unwrap(); // == "example.com"
    // let path = consumer.next_until_space().unwrap(); // == "/rust"
    //
    // assert_eq!(&protocol, "http");
    // assert_eq!(&host, "example.com");
    // assert_eq!(&path, "/rust");
    // ```
    pub fn next_until(&mut self, s: &str) -> Option<String> {
        let mut n = 0;
        // 
        loop {
            match self.peek_n(s.len() + n) {
                Some(peeked) => {
                    if &peeked[n..] == s {
                        self.next_n(n);
                        return Some(peeked[..n].to_string());
                    }
                }
                None => return None,
            }
            n += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_until() {
        let mut con = Consumer::new("https://ja.wikipedia.org/wiki/Uniform_Resource_Locator");
        
        let protocol = con.next_until("://").unwrap();
        con.next_n("://".len());
        let host = con.next_until("/").unwrap();
        let path = con.next_until_space().unwrap();

        assert_eq!(&protocol, "https");
        assert_eq!(&host, "ja.wikipedia.org");
        assert_eq!(&path, "/wiki/Uniform_Resource_Locator");
    }
}
