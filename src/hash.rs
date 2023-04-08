use std::collections::HashMap;
use std::path::Path;

use super::lex;
use super::parse;

pub fn hash_document(path: &str, lowercase: bool) -> HashMap<String, u32> {
    //hashes a document into hasmap of its individual words and how many times they occur
    let content = parse::get_contents_from_file(&path, lowercase)
        .unwrap()
        .chars()
        .collect::<Vec<_>>();

    let mut hashed_document: HashMap<String, u32> = HashMap::new();

    for token in lex::Lexer::new(&content) {
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
    //a hasmap of a files path to its previosly hashed document of words and related frequencies
    let home_dir = Path::new(path);

    let mut file_paths: Vec<String> = Vec::new();
    parse::visit_dirs_and_get_supported_extensions(&home_dir, &mut file_paths);

    let mut all_hashed_documents: HashMap<String, HashMap<String, u32>> = HashMap::new();
    for file_path in file_paths {
        all_hashed_documents.insert(file_path.clone(), hash_document(&file_path, lowercase));
        println!("{}", &file_path);
    }

    return all_hashed_documents;
}

fn _print_all_hashed(docs: &HashMap<String, HashMap<String, u32>> ){
    let docs: Vec<(&String, &HashMap<String, u32>)> = docs.into_iter().collect();
    for doc in docs {
        println!("{}: Count - {}", doc.0, doc.1.len());
    }
}

pub fn print_hashed(doc: &HashMap<String, u32>){
    let mut doc = doc.iter().collect::<Vec<_>>();
    doc.sort_by_key(|(_, b)| *b);
    doc.reverse();
    for token in doc.iter() {
        println!("{}: Count - {}", token.0, token.1);
    }
}