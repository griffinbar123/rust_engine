use xml;
use std::fs::{self, File};
use std::io::{self};
use std::path::{Path};
use std::collections::HashMap;

use super::Lexer;

pub fn visit_dirs_and_get_xhtml_extensions(dir: &Path, file_paths: &mut Vec<String>) {
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

pub fn print_tfidf_vecs(doc: &Vec<(String, f64)>){
    for token in doc.iter().take(5) {
        println!("{}: TFIDF - {}", token.0, token.1);
    }
}


pub fn get_raw_xhtml(file_path: &str) -> String{
    let file = fs::read_to_string(&file_path).unwrap();
    // eprintln!("{}",&file);
    return file;
}

pub fn get_string_from_xhtml_file(file_path: &str, lowercase: bool) -> io::Result<String> {
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

pub fn search(query: &Vec<String>, hashed_docs: &HashMap<String, HashMap<String, u32>>) -> Option<Vec<(String, f64)>> {

    let mut document_scores = Vec::new();
    for (path, doc) in hashed_docs {
        let mut tf_idf = 0.0;
        for token in query {
            tf_idf += get_idf(&token, &hashed_docs) * get_tf(&token, &doc);
        }
        document_scores.push((path.to_string(), tf_idf));
    }


    document_scores
    .sort_by(|a, b| match b.1.partial_cmp(&a.1) {
        Some(document_scores) => document_scores,
        None => std::cmp::Ordering::Less,
    });
    return Some(document_scores[..40].to_vec());
}

fn get_idf(query: &str, hashed_docs: &HashMap<String, HashMap<String, u32>>) -> f64 {
    let m = hashed_docs.values().filter(|tf| tf.contains_key(query)).count().max(1) as f64;
    let size = hashed_docs.len()as f64;
    return (size/m).log10();
}

fn get_tf(query: &str, hashed_doc: &HashMap<String, u32>) -> f64 {
    let total_frequency = hashed_doc.get(query).cloned().unwrap_or(0) as f64;
    let total_words = hashed_doc.iter().map(|(_, t)| *t).sum::<u32>() as f64;
    return total_frequency/total_words;
}

pub fn parse_string(query: &str, lowercase: bool)->Vec<String>{
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