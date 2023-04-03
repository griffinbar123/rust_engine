use std::path::{Path};
use std::collections::HashMap;

pub mod parse;

#[derive(Debug)]
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

pub fn hash_document(path: &str, lowercase: bool) -> HashMap<String, u32> {
    let content = parse::get_string_from_xhtml_file(&path, lowercase)
        .unwrap()
        .chars()
        .collect::<Vec<_>>();

    let mut hashed_document: HashMap<String, u32> = HashMap::new();

    for token in Lexer::new(&content) {
        let token = token.iter().collect::<String>();
        if hashed_document.contains_key(&token) {
            let token_count = hashed_document.get_mut(&token).unwrap().clone();
            // println!("{}: Count - {}", &token, token_count);
            hashed_document.insert(token, token_count+1);
        } else {
            hashed_document.insert(token, 1);
        }
    }
    return hashed_document;
}

pub fn hash_all_documents(path: &str, lowercase: bool) -> HashMap<String, HashMap<String, u32>> {
    let home_dir = Path::new(path);

    let mut file_paths: Vec<String> = Vec::new();
    parse::visit_dirs_and_get_xhtml_extensions(&home_dir, &mut file_paths);

    let mut all_hashed_documents: HashMap<String, HashMap<String, u32>> = HashMap::new();
    for file_path in file_paths {
        all_hashed_documents.insert(file_path.clone(), hash_document(&file_path, lowercase));
        println!("{}", &file_path);
    }

    return all_hashed_documents;
}

// fn print_all_hashed(docs: &HashMap<String, HashMap<String, u32>> ){
//     let docs: Vec<(&String, &HashMap<String, u32>)> = docs.into_iter().collect();
//     for doc in docs {
//         println!("{}: Count - {}", doc.0, doc.1.len());
//     }
// }

pub fn print_hashed(doc: &HashMap<String, u32>){
    let mut doc = doc.iter().collect::<Vec<_>>();
    doc.sort_by_key(|(_, b)| *b);
    doc.reverse();
    for token in doc.iter() {
        println!("{}: Count - {}", token.0, token.1);
    }
}

// fn get_user_input(prompt: &str) -> String {
//     let mut s=String::new();
//     print!("{prompt}");
//     io::stdout().flush().expect("did not flush");
//     io::stdin().read_line(&mut s).expect("Did not enter a correct string");
//     if let Some('\n')=s.chars().next_back() {
//         s.pop();
//     }
//     if let Some('\r')=s.chars().next_back() {
//         s.pop();
//     }
//     return s;
// }