use std::fs::File;
use std::collections::HashMap;
use std::path::Path;
use tiny_http::{Request, Response, Header, Method};
use serde_json;
use serde::{self, Deserialize, Serialize};

use super::hash;
use super::parse;
use super::engine;


#[derive(Serialize, Deserialize)]
struct Message {
    message: String,
}

pub fn start_server(mut all_hashed_documents: HashMap<String, HashMap<String, u32>>, dir: &str){

    let server = tiny_http::Server::http("127.0.0.1:8080").unwrap();

    eprintln!("http://127.0.0.1:8080");

    let mut directory = dir.to_string();

    loop {
        // blocks until the next request is received
        let _request = match server.recv() {
            Ok(rq) => {
                let x = handle_connection(rq, &all_hashed_documents, &directory, dir);
                match x {
                    None => continue,
                    Some(x) => {
                        if x != directory {
                            directory = x;
                            all_hashed_documents = hash::hash_all_documents(&directory, true);
                        }
                    }
                }
            },
            Err(e) => { println!("error recieving request: {}", e); break }
        };
    }
}

fn handle_connection(request: Request, _all_hashed_documents: &HashMap<String, HashMap<String, u32>>, directory: &str, base_directory: &str) -> Option<String> {
    let home= "src/home.html".to_string();
    let js = "src/index.js".to_string();
    let css = "src/style.css".to_string();
    

    eprintln!("RECIEVED REQUEST! method: {} - url: {}", request.method(), request.url());

    //handle serving html file to user 
    if request.url().starts_with("/?") {
        //document is filename retrieved from url. different from api
        let base = String::from("/?doc=");

        let doc = request.url().to_string();
        let doc = doc.chars().take(0).chain(doc.chars().skip(base.len())).collect::<String>();
        match &request.method() {
            Method::Get => {
                let content_type = "text/html; charset=utf-8";

                eprintln!("{}", doc);

                respond_with_file(request, &doc, content_type);
                return None;
            }
            _=> {
                let content_type = "text/html; charset=utf-8";
                respond_with_file(request, &home, content_type);
            }
        }
        return None;
    }

    match (&request.method(), request.url()) {
        (Method::Post, "/api/change-dir") => {
            let content_type = "text/html; charset=utf-8";
            //changes current directory you are looking at

            let (body, request) = get_string_from_request(request);
            let doc:Message = serde_json::from_str(&body).unwrap();

            eprintln!("{}", doc.message);

            respond_with_file(request, &home, content_type);
            return Some(doc.message);
        }
        (Method::Post, "/api/query") => {
            //handles query from user for files
            let content_type = "application/json; charset=utf-8";
            let (body, request) = get_string_from_request(request);

            let query:Message = serde_json::from_str(&body).unwrap();
            let mut query = query.message;

            if query.len() == 0 {
                query = "a".to_string();
            }
            let query = parse::tokenize_string(&query, true);
            let results = engine::search(&query, _all_hashed_documents).unwrap();

            let results = results.iter().map(|(a, _)| a.to_string()).collect::<Vec<String>>();
            respond_with_multiple_strings(request, &results, content_type);
        }
        (Method::Get, "/api/current-dir") => {
            //responds with the current directoy that is indexed
            let content_type = "application/json; charset=utf-8";

            respond_with_string(request, &directory, content_type);
        }
        (Method::Get, "/api/all-dirs") => {
            //responds with the current directoy that is indexed
            let content_type = "application/json; charset=utf-8";

            let directories: Vec<String> = parse::get_local_directories(base_directory);
            respond_with_multiple_strings(request, &directories, content_type);
        }
        (Method::Get, "/index.js") => {
            let content_type = "text/javascript; charset=utf-8";
            respond_with_file(request, &js, content_type);
        }
        (Method::Get, "/style.css") => {
            let content_type = "text/css; charset=utf-8";
            respond_with_file(request, &css, content_type);
        }
        (Method::Get, "/") | (Method::Get, "/index") => {
            let content_type = "text/html; charset=utf-8";
            respond_with_file(request, &home, content_type);
        }
        _ => {
            let content_type = "text/html; charset=utf-8";
            respond_with_file(request, &home, content_type);
        }
    };
    return None;
}

fn get_string_from_request(mut request: Request) -> (String, Request) {
    //function thats the body of a request and returns it and the request itself
    let mut buf = Vec::new();
    request.as_reader().read_to_end(&mut buf).expect("unwrap this pls ");
    let body = std::str::from_utf8(&buf).unwrap().to_string();
    return (body, request);
}

fn respond_with_file(request: Request, file: &str, content_type: &str) {
    //function that takes a filename and gives that file as a response to the request
    let response = Response::from_file(File::open(&Path::new(file)).unwrap())
        .with_header(Header::from_bytes("Content-Type", content_type)
        .expect("That we had no proble unwrapp headers"));
    request.respond(response).expect("Did send request (file)");
}

fn respond_with_string(request: Request, message: &str, content_type: &str) {
    //function that takes a string and turns it into a json message the client can read
    let mut json =  String::from("{\"message\" : \"");
    json.push_str(message);
    json.push_str("\"}");

    let response = Response::from_string(json).
        with_header(Header::from_bytes("Content-Type", content_type)
        .expect("That we had no proble unwrapp headers"));
    request.respond(response).expect("Did send request (string)");
}

fn respond_with_multiple_strings(request: Request, results:&Vec<String>, content_type: &str) {
    //function that responds to with a list of strings 
    let mut json =  String::from("{\"paths\" : [");
    for i in 0..results.len()-1 {
        json.push('"');
        let result = &results[i];
        json.push_str(result);
        json.push_str("\",");
    }
    json.push('"');
    json.push_str(&results[results.len()-1]);
    json.push_str("\"]}");

    let response = Response::from_string(&json)
    .with_header(Header::from_bytes("Content-Type", content_type)
    .expect("That we had no problem unwrapping headers"));

    request.respond(response).expect("Did send request (file)");
} 

