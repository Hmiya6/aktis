
// support tags
//
// <html>, <head>, <body>
// <title> 
// <h1>, <h2>, <h3>, <h4>, <h5> -> #
// <p>
// <a> -> []()
// <ol>, <ul>, <li>
// <div>
// <hr> -> ---
// <code> -> ```
//

// TODO:
// - split off tag name state from 

use crate::utils::consumer::Consumer;



// reference: https://html.spec.whatwg.org/multipage/parsing.html#tokenization
struct Tokenizer {
    con: Consumer,
    state: StateType,
    tokens: Vec<Token>,
    current: Option<Token>,
    current_attr: Attribute,
}

impl Tokenizer {
    
    pub fn new(src: &str) -> Self {
        Self {
            con: Consumer::new(src),
            state: StateType::Data,
            tokens: vec![],
            current: None,
            current_attr: Attribute::new(),
        }
    }

    pub fn execute(&mut self) -> Vec<Token> {
        while self.con.peek_char().is_some() {
            match self.state {
                StateType::Data => self.data(),
                StateType::TagOpen => self.tag_open(),
                StateType::EndTagOpen => self.end_tag_open(),
                StateType::TagName => self.tag_name(),
                StateType::BeforeAttributeName => self.before_attr_name(),
                StateType::AttributeName => self.attr_name(),
                StateType::BeforeAttributeValue => self.before_attr_val(),
                StateType::AttributeValueQuoted => self.attr_val_quoted(),
                StateType::AttributeValueUnquoted => self.attr_val_unquoted(),
                StateType::AfterAttributeValue => self.after_attr_val(),
                StateType::SelfClosingStartTag => self.self_closing_start_tag(),
                StateType::MarkupDeclarationOpen => self.markup_declaration_open(),
                StateType::Comment => self.comment(),
                StateType::Doctype => self.doctype(),
            }
        }
        
        // self.tokens.push(std::mem::replace(&mut self.current, Token::default()));
        std::mem::take(&mut self.tokens)
    }

    fn data(&mut self) {
        if self.current.is_none() {
            self.current = Some(Token::new(TokenType::Content));
        }
        // consume the next input character
        match self.con.next_char() {
            Some(c) => match c {
                '<' => {
                    // switch to the tag open state
                    self.state = StateType::TagOpen;
                    // TODO: is really the following code needed?
                    // self.tokens.push(std::mem::take(&mut self.current));
                },
                // emit the current input charater as a character token
                _ => self.current.as_mut().unwrap().push_data(c),
            }
            None => return,
        }
    }

    fn tag_open(&mut self) {
        if let Some(token) = &mut self.current {
            if token.data.is_some() {
                self.tokens.push(self.current.take().unwrap());
            }
        }
        self.current = Some(Token::new(TokenType::StartTag));
        // consume the next input character
        match self.con.peek_char() {
            Some(c) => match c {
                '!' => {
                    // switch to the markup declaration open state
                    self.con.next_char().unwrap();
                    self.state = StateType::MarkupDeclarationOpen;
                },
                '/' => {
                    // switch to the end tag open state
                    self.con.next_char().unwrap();
                    self.state = StateType::EndTagOpen;
                },
                // 
                c if c.is_alphabetic() => self.state = StateType::TagName,
                _ => panic!("Error: Invalid first character of tag name"),
            },
            None => panic!("Error: EOF before tag name"),
        }
    }

    fn end_tag_open(&mut self) {
        self.current.as_mut().unwrap().set_type(TokenType::EndTag);
        match self.con.peek_char() {
            Some(c) => match c {
                '>' => panic!("Error: Missing end tag name"),
                c if c.is_alphabetic() => self.state = StateType::TagName,
                _ => panic!("Error: Invalid first character of tag name"),
            }
            None => panic!("Error: EOF before tag name"),
        }

    }

    fn tag_name(&mut self) {
        match self.con.next_char() {
            Some(c) => match c {
                ' ' => self.state = StateType::BeforeAttributeName,
                '/' => self.state = StateType::SelfClosingStartTag,
                '>' => {
                    self.state = StateType::Data;
                    self.tokens.push(self.current.take().unwrap());
                },
                _ => self.current.as_mut().unwrap().push_name(c),
            }
            None => panic!("Error: EOF in tag"),
        }
    }
    
    // ref: https://html.spec.whatwg.org/multipage/parsing.html#before-attribute-name-state
    fn before_attr_name(&mut self) {
        match self.con.peek_char() {
            Some(c) => match c {
                ' ' => self.con.skip_space(),
                '=' => panic!("Error: Unexpected equals sign before attribute name"),
                _ => self.state = StateType::AttributeName,
            }
            //
            None => panic!("Error: Unexpected EOF"),
        }
    }
    
    // ref: https://html.spec.whatwg.org/multipage/parsing.html#attribute-name-state
    fn attr_name(&mut self) {
        match self.con.next_char() {
            Some(c) => match c {
                c if c.is_alphabetic() => self.current_attr.push_name(c),
                '=' => self.state = StateType::BeforeAttributeValue,
                _ => panic!("Error: Unexpected character in attribute name"),
            }
            // EOF -> Reconsume in the after attribute name state
            None => panic!("Error: Unexpected EOF"),
        }
    }
        
    // ref: https://html.spec.whatwg.org/multipage/parsing.html#before-attribute-value-state
    fn before_attr_val(&mut self) {
        match self.con.peek_char() {
            Some(c) => match c {
                ' ' => self.con.skip_space(),
                '\'' | '"' => {
                    self.state = StateType::AttributeValueQuoted;
                    self.con.next_char().unwrap();
                },
                '>' => panic!("Error: Missing attribute value"),
                _ => self.state = StateType::AttributeValueUnquoted,
            }
            None => panic!("Error: Unexpected EOF"),
        }
    }
    
    // ref: https://html.spec.whatwg.org/multipage/parsing.html#attribute-value-(double-quoted)-state
    // ref: https://html.spec.whatwg.org/multipage/parsing.html#attribute-value-(single-quoted)-state
    fn attr_val_quoted(&mut self) {
        match self.con.next_char() {
            Some(c) => match c {
                '\'' | '"' => {
                    self.state = StateType::AfterAttributeValue;
                    let attr = std::mem::replace(&mut self.current_attr, Attribute::new());
                    self.current.as_mut().unwrap().push_attr(attr);
                },
                _ => self.current_attr.push_val(c),
                // _ => panic!("Error: Invalid character in attribute value; `{}`", c),
            }
            None => panic!("Error: EOF in tag"),
        }
    }

    // ref: https://html.spec.whatwg.org/multipage/parsing.html#attribute-value-(unquoted)-state
    fn attr_val_unquoted(&mut self) {
        match self.con.next_char() {
            Some(c) => match c {
                ' ' | '\t' | '\r' | '\n' => self.state = StateType::BeforeAttributeName,
                '>' => {
                    self.state = StateType::Data;
                    self.tokens.push(self.current.take().unwrap());
                },
                c if c.is_alphanumeric() => self.current_attr.push_val(c),
                _ => panic!("Error: Unexpected character in unquoted attribute value"),
            }
            None => panic!("Error: EOF in tag"),
        }
    }


    // ref: https://html.spec.whatwg.org/multipage/parsing.html#after-attribute-value-(quoted)-state
    fn after_attr_val(&mut self) {
        match self.con.next_char() {
            Some(c) => match c {
                ' ' | '\r' | '\t' | '\n' => self.state = StateType::BeforeAttributeName,
                '/' => {
                    self.state = StateType::SelfClosingStartTag;
                    self.current.as_mut().unwrap().set_self_closing(true);
                },
                '>' => {self.state = StateType::Data},
                _ => panic!("Error: Missing whitespace between attributes"),
            }
            None => panic!("Error: EOF in tag"),
        }
    }
    
    // ref: https://html.spec.whatwg.org/multipage/parsing.html#after-attribute-value-(quoted)-state
    fn self_closing_start_tag(&mut self) {
        match self.con.next_char() {
            Some(c) => match c {
                '>' => {
                    self.current.as_mut().unwrap().set_self_closing(true);
                    self.state = StateType::Data;
                    self.tokens.push(self.current.take().unwrap());
                }
                _ => panic!("Error: Unexpected solidus in tag"),
            }
            None => panic!("Error: EOF in tag"),
        }
    }
    
    // ref: https://html.spec.whatwg.org/multipage/parsing.html#after-attribute-value-(quoted)-state
    fn markup_declaration_open(&mut self) {

        let doctype_len = 7; // == "DOCTYPE".len();
        if let Some(s) = self.con.peek_n(doctype_len) {
            if s == "DOCTYPE" {
                self.con.next_n(doctype_len).unwrap();
                self.state = StateType::Doctype;
                self.current.as_mut().unwrap().set_type(TokenType::Doctype);
            }
        }

        let comment_tag_len = 2; // == "--".len(); // <- already read "<!"
        if let Some(s) = self.con.next_n(comment_tag_len) {
            if s == "--" {
                self.state = StateType::Comment;
                self.current.as_mut().unwrap().set_type(TokenType::Comment);
            }
        }
        panic!("Error: Incorrectly opened comment");
    }

    fn comment(&mut self) {
        let end_comment = "-->";
        match self.con.next_until(end_comment) {
            Some(s) => {
                self.con.next_n(end_comment.len()).unwrap();
                s.chars().for_each(|c| self.current.as_mut().unwrap().push_data(c));
            }
            None => panic!("Error: EOF in comment tag"),
        }
    }

    fn doctype(&mut self) {
        if let Some(c) = self.con.next_char() {
            if !"\t\r\n ".contains(c) {
                panic!("Error: Missing whitespece before DOCTYPE name");
            }
        } else {
            panic!("Error: EOF in DOCTYPE");
        }

        match self.con.next_until(">") {
            Some(s) => {
                self.con.next().unwrap();
                s.chars().for_each(|c| {
                    if !c.is_alphanumeric() {
                        panic!("Error: Invalid character in DOCTYPE name");
                    }
                    self.current.as_mut().unwrap().push_name(c);
                });
            }
            None => panic!("Error: EOF in DOCTYPE name"),
        }
    }
}

enum StateType {
    Data,
    TagOpen,
    EndTagOpen,
    TagName,
    BeforeAttributeName,
    AttributeName,
    BeforeAttributeValue,
    AttributeValueQuoted,
    AttributeValueUnquoted,
    AfterAttributeValue,
    SelfClosingStartTag,
    MarkupDeclarationOpen,
    Comment,
    Doctype,
}


struct Token {
    token_type: TokenType,
    data: Option<String>,
    name: Option<String>,
    attr: Vec<Attribute>,
    self_closing: bool,
}

impl Default for Token {
    fn default() -> Self {
        Self::new(TokenType::Content)
    }
}

impl Token {
    pub fn new(token_type: TokenType) -> Self {
        Self {
            token_type,
            data: None,
            name: None,
            attr: vec![],
            self_closing: false,
        }
    }

    pub fn set_type(&mut self, token_type: TokenType) {
        self.token_type = token_type;
    }

    pub fn push_data(&mut self, c: char) {
        match &mut self.data {
            Some(inner) => inner.push(c),
            None => self.data = Some(String::from(c)),
        }
    }

    pub fn push_name(&mut self, c: char) {
        match &mut self.name {
            Some(inner) => inner.push(c),
            None => self.name = Some(String::from(c)),
        }
    }

    pub fn push_attr(&mut self, attr: Attribute) {
        self.attr.push(attr);
    }

    pub fn set_self_closing(&mut self, b: bool) {
        self.self_closing = b;
    }
}

#[derive(Debug)]
enum TokenType {
    Doctype,
    Comment,
    StartTag,
    EndTag,
    Content,
}

#[derive(Debug)]
struct Attribute {
    name: String,
    value: String,
}

impl Attribute {

    pub fn new() -> Self {
        Self {
            name: String::new(),
            value: String::new(),
        }
    }

    pub fn push_name(&mut self, c: char) {
        self.name.push(c);

    }

    pub fn push_val(&mut self, c: char) {
        self.value.push(c);
    }
}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_tokenize() {
        tokenize("<body><h1>Hello</h1></body>");
        tokenize("<img src='hello.png' alt='hello image'/>")
    }

    fn tokenize(src: &str) {
        // let src = "<body><h1>Hello</h1></body>";
        let mut tokenizer = Tokenizer::new(src);
        let tokens = tokenizer.execute();

        let types = tokens.iter().to_owned()
            .map(|token| &token.token_type)
            .collect::<Vec<_>>();
        // assert_eq!(types, vec![TokenType::StartTag, TokenType::StartTag, TokenType::Content, TokenType::EndTag, TokenType::EndTag]);

        let names = tokens.iter().to_owned()
            .map(|token| token.name.as_ref())
            .collect::<Vec<_>>();

        let datas = tokens.iter().to_owned()
            .map(|token| token.data.as_ref())
            .collect::<Vec<_>>();

        let attrs = tokens.iter().to_owned()
            .map(|token| &token.attr)
            .collect::<Vec<&Vec<_>>>();

        println!("{:?}", types);
        println!("{:?}", names);
        println!("{:?}", datas);
        println!("{:?}", attrs);
    }
}





