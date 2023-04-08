use xml;
use std::fs::{self, File};
use std::path::{Path};
use super::lex;

pub fn visit_dirs_and_get_supported_extensions(dir: &Path, file_paths: &mut Vec<String>) {
    if dir.is_dir() {
        let dir = fs::read_dir(dir).unwrap();
        for entry in dir {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                visit_dirs_and_get_supported_extensions(&path, file_paths);
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

pub fn get_local_directories(dir: &str) -> Vec<String> {
    let mut dirs:Vec<String> = Vec::new();
    dirs.push(dir.to_string());
    get_local_directories_recursively(&Path::new(dir), &mut dirs);

    return dirs;
}


fn get_local_directories_recursively(dir: &Path, file_paths: &mut Vec<String>) {
    if dir.is_dir() {
        let dir = fs::read_dir(dir).unwrap();
        for entry in dir {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                let temp = path.display().to_string();
                if !temp.contains("git") { //filter out git files
                    file_paths.push(temp);
                    get_local_directories_recursively(&path, file_paths);
                }
            } else {
                continue
            }
        }
    }
}


pub fn get_raw_xhtml(file_path: &str) -> String{
    //helpful function to just see the contents of an xhtml file
    let file = fs::read_to_string(&file_path).unwrap();
    // eprintln!("{}",&file);
    return file;
}

pub fn get_contents_from_file(file_path: &str, lowercase: bool) -> Option<String> {
    let ext = match Path::new(file_path).extension() {
        Some(ext) => ext,
        None => return None,
    };
    if ext == "xhtml" {
        return get_contents_from_xhtml_file(file_path, lowercase);
    } else {
        return None;
    }
}

fn get_contents_from_xhtml_file(file_path: &str, lowercase: bool) -> Option<String> {
    //this gets the contents from an xhtml file that we use
    let file = File::open(file_path).unwrap();
    let er = xml::EventReader::new(file);
    let mut content = String::new();

    for event in er.into_iter(){
        if let xml::reader::XmlEvent::Characters(text) = event.unwrap() {
            content.push_str(&text);
            content.push_str(" ");
        }
    }
    if lowercase {return Some(content.to_ascii_lowercase())}
    return Some(content);
}

pub fn tokenize_string(query: &str, lowercase: bool)->Vec<String>{
    //takes a string and calls the lexer on it to tokenize it
    let query = query.to_string().chars().collect::<Vec<_>>();

    let query = lex::Lexer::new(&query);
    
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