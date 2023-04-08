use std::io::{self, Write};

pub struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a [char]) -> Lexer<'a> {
        Self {content}
    }
    fn skip_whitespace(&mut self) {
        while self.content.len() > 0 && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
        } 
    }

    fn skip_junk(&mut self) {
        while self.content.len() > 0 && (!self.content[0].is_alphabetic() && !self.content[0].is_numeric()) {
            self.content = &self.content[1..];
        } 
    }

    pub fn next_token(&mut self) -> Option<&'a [char]>{
        self.skip_whitespace();
        self.skip_junk();
    
        if self.content.len() == 0 {// checl if no more charactrs left
            return None;
        }
        if self.content[0].is_alphabetic(){ // get a token
            let mut i = 1;
            while i < self.content.len() && self.content[i].is_alphanumeric(){
                i+=1;
            }
            let token = &self.content[0..i];
            self.content = &self.content[i..];
            return Some(token);
        }

        if self.content[0].is_numeric(){ // get a token
            let mut i = 1;
            while i < self.content.len() && self.content[i].is_numeric(){
                i+=1;
            }
            let token = &self.content[0..i];
            self.content = &self.content[i..];
            return Some(token);
        }
        panic !("Handle this");
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a [char];
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}


fn _get_user_input(prompt: &str) -> String {
    //useful input to get user input easily
    let mut s=String::new();
    print!("{prompt}");
    io::stdout().flush().expect("did not flush");
    io::stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    return s;
}