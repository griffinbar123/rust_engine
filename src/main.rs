use xml;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path};
use std::collections::HashMap;
use std::process::exit;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    fn new(content: &'a [char]) -> Lexer<'a> {
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

    fn next_token(&mut self) -> Option<&'a [char]>{
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

fn hash_document(path: &str, lowercase: bool) -> HashMap<String, u32> {
    let content = get_string_from_xhtml_file(&path, lowercase)
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

fn hash_all_documents(path: &str, lowercase: bool) -> HashMap<String, HashMap<String, u32>> {
    let home_dir = Path::new(path);

    let mut file_paths: Vec<String> = Vec::new();
    visit_dirs_and_get_xhtml_extensions(&home_dir, &mut file_paths);

    let mut all_hashed_documents: HashMap<String, HashMap<String, u32>> = HashMap::new();
    for file_path in file_paths {
        all_hashed_documents.insert(file_path.clone(), hash_document(&file_path, lowercase));
    }

    return all_hashed_documents;
}

// fn print_all_hashed(docs: &HashMap<String, HashMap<String, u32>> ){
//     let docs: Vec<(&String, &HashMap<String, u32>)> = docs.into_iter().collect();
//     for doc in docs {
//         println!("{}: Count - {}", doc.0, doc.1.len());
//     }
// }

// fn print_hashed(doc: &HashMap<String, u32>){
//     let mut doc = doc.iter().collect::<Vec<_>>();
//     doc.sort_by_key(|(_, b)| *b);
//     doc.reverse();
//     for token in doc.iter().take(10) {
//         println!("{}: Count - {}", token.0, token.1);
//     }
// }

fn print_tfidf_vecs(doc: &Vec<(String, f64)>){
    for token in doc.iter().take(5) {
        println!("{}: TFIDF - {}", token.0, token.1);
    }
}
fn main() {
    println!("Hashing........");

    let all_hashed_documents = hash_all_documents("docs.gl/gl4", true);
    
    loop {
        let query = get_user_input("Enter command or query: ");
        if query == "quit" {
            break;
        }
        if query.len() > 5 && &query[..5] == "serve" {
            do_web_stuff(&query[6..]);
            break;
        }
        let query = parse_string(&query, true);
        let search_results = search(&query, &all_hashed_documents);
        print_tfidf_vecs(&search_results);
        // do_web_stuff(&search_results[0].0)
    }
    
        
}

fn do_web_stuff(query: &str){
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();

            handle_connection(stream, &query);
        }
}

fn handle_connection(mut stream: TcpStream, query: &str) {
    eprintln!("{}", query);
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

        let status_line = "HTTP/1.1 200 OK";
        let contents = get_raw_xhtml(&query);
        let length = contents.len();
    
        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
        stream.write_all(response.as_bytes()).unwrap();
}

fn get_user_input(prompt: &str) -> String {
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

fn parse_string(query: &str, lowercase: bool)->Vec<String>{
    let query = query.to_string().chars().collect::<Vec<_>>();

    // let binding = query.to_string().chars().collect::<Vec<_>>();
    let query = Lexer::new(&query);
    
    let mut final_query = Vec::new();
    for token in query{
        if lowercase {
            final_query.push(token.iter().collect::<String>().to_ascii_lowercase());
        } else {
            final_query.push(token.iter().collect::<String>());
        }
    }
    return final_query;
}

// fn show_file(f: &XhtmlFile) {
//     println!("{:?}", f.content)
// }

fn search(query: &Vec<String>, hashed_docs: &HashMap<String, HashMap<String, u32>>) -> Vec<(String, f64)> {
    let idf_scores = get_idf_scores(query, hashed_docs);

    let mut document_scores: Vec<(String, f64)> = Vec::new();
    hashed_docs.iter().for_each(|f| {
        document_scores.push((f.0.to_string(), good_get_tf_idf_score(f.1, &query, &idf_scores)));
    });

    document_scores
    .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or_else(|| {
        eprintln!("No matches found");
        exit(1)
    }));
    return document_scores;
}


fn good_get_tf_idf_score(hashed_docs: &HashMap<String, u32>, query: &Vec<String>, idf_scores: &Vec<u32>) -> f64{
    let mut tf_idf_score: f64 = 0.0;
    for (i,item) in query.iter().enumerate() {
        let tf: u32 = match hashed_docs.get(item){
            Some(v) => *v,
            None => 0,
        };
        tf_idf_score += tf as f64 / idf_scores[i] as f64;
    }

    return tf_idf_score;
}

fn get_idf_scores(query: &Vec<String>, hashed_docs: &HashMap<String, HashMap<String, u32>>) -> Vec<u32>{
    let mut idf_scores: Vec<u32> = Vec::new();
    let hashed_docs: Vec<(_, &HashMap<String, u32>)> = hashed_docs.into_iter().collect();
    for token in query {
        idf_scores.push(
            hashed_docs.iter()
                .filter(|f| f.1.contains_key(token))
                .count().try_into().unwrap()
        );
    }
    return idf_scores;
}

fn visit_dirs_and_get_xhtml_extensions(dir: &Path, file_paths: &mut Vec<String>) {
    if dir.is_dir() {
        let dir = fs::read_dir(dir).unwrap();
        for entry in dir {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                visit_dirs_and_get_xhtml_extensions(&path, file_paths);
            } else {
                let ext = path.extension();
                let ext = match ext {
                    Some(ext) => ext,
                    _ => continue,
                };
                if ext == "xhtml" {
                    file_paths.push(path.as_path().display().to_string());
                }
            }
        }
    }
}

// fn print_input_file(file_path: &str) {
//     println!("{}", fs::read_to_string(file_path).unwrap());
// }


fn get_raw_xhtml(file_path: &str) -> String{
    let file = fs::read_to_string(&file_path).unwrap();
    return file;
}

fn get_string_from_xhtml_file(file_path: &str, lowercase: bool) -> io::Result<String> {
    let file = File::open(file_path)?;
    let er = xml::EventReader::new(file);
    let mut content = String::new();

    for event in er.into_iter(){
        if let xml::reader::XmlEvent::Characters(text) = event.unwrap() {
            content.push_str(&text);
            content.push_str(" ");
            // println!("CONTENT: {}", res);
        }
    }
    if lowercase {return Ok(content.to_ascii_lowercase())}
    return Ok(content);

}

