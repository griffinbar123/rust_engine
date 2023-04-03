use std::fs::File;
use std::collections::HashMap;
use std::path::Path;
use tiny_http::{Request, Response, Header, Method};
use serde_json;
use serde::{self, Deserialize, Serialize};
pub mod lex;
#[derive(Serialize, Deserialize)]
struct Message {
    message: String,
}

const DIRECTORY: &'static str = "docs.gl/gl4";

fn main() {

    let all_hashed_documents = lex::hash_all_documents(DIRECTORY, true);

    do_web_stuff(all_hashed_documents);
        
}

fn do_web_stuff(mut all_hashed_documents: HashMap<String, HashMap<String, u32>>){

    let server = tiny_http::Server::http("127.0.0.1:8080").unwrap();

    eprintln!("http://127.0.0.1:8080");

    let mut directory = DIRECTORY.to_string();

    loop {
        // blocks until the next request is received
        let _request = match server.recv() {
            Ok(rq) => {
                let x = handle_connection(rq, &all_hashed_documents, &directory);
                if x == "none" { continue }
                else if x != directory {
                    directory = x;
                    all_hashed_documents = lex::hash_all_documents(&directory, true);
                }
            },
            Err(e) => { println!("error: {}", e); break }
        };
    }
}

fn handle_connection(mut request: Request, _all_hashed_documents: &HashMap<String, HashMap<String, u32>>, directory: &str) -> String {
    let home= "src/home.html".to_string();
    let js = "src/index.js".to_string();
    let css = "src/style.css".to_string();
    let base = String::from("/?doc=");

    eprintln!("RECIEVED REQUEST! method: {} - url: {}", request.method(), request.url());

    //handle serving html
    if request.url().starts_with("/?") {
        let doc = request.url().to_string();
        let doc = doc.chars().take(0).chain(doc.chars().skip(base.len())).collect::<String>();
        match &request.method() {
            Method::Get => {
                let content_type = "text/html; charset=utf-8";
                // let mut base = String::from(DIRECORY);
                eprintln!("{}", doc);

                respond_with_file(request, &doc, content_type, true);
                return "none".to_string();
            }
            _=> {
                let content_type = "text/html; charset=utf-8";
                respond_with_file(request, &home, content_type, false);
            }
        }
        return "none".to_string();
    }

    match (&request.method(), request.url()) {
        (Method::Post, "/api/change-dir") => {
            let content_type = "text/html; charset=utf-8";
            // let mut base = String::from(DIRECTORY);

            let (body, request) = get_string_from_request(request);
            let doc:Message = serde_json::from_str(&body).unwrap();

            eprintln!("{}", doc.message);

            respond_with_file(request, &home, content_type, true);
            return doc.message;
        }
        (Method::Post, "/api/query") => {
            let content_type = "application/json; charset=utf-8";
            let (body, request) = get_string_from_request(request);

            let query:Message = serde_json::from_str(&body).unwrap();
            let query = query.message;

            if query.len() == 0 {
                let content_type = "text/html; charset=utf-8";
                respond_with_file(request, &home, content_type, false);
                return "none".to_string();
            }
            let query = lex::parse::parse_string(&query, true);
            let results = lex::parse::search(&query, _all_hashed_documents).unwrap();

            handle_query(request, &results, content_type);
        }
        (Method::Get, "/api/current-dir") => {
            let content_type = "application/json; charset=utf-8";

            respond_with_string(request, &directory, content_type, true);
        }
        (Method::Get, "/index.js") => {
            let content_type = "text/javascript; charset=utf-8";
            respond_with_file(request, &js, content_type, false);
        }
        (Method::Get, "/style.css") => {
            let content_type = "text/css; charset=utf-8";
            respond_with_file(request, &css, content_type, false);
        }
        (Method::Get, "/") | (Method::Get, "/index") => {
            let content_type = "text/html; charset=utf-8";
            respond_with_file(request, &home, content_type, false);
        }
        _ => {
            let content_type = "text/html; charset=utf-8";
            respond_with_file(request, &home, content_type, false);
        }
    };
    return "none".to_string();
}

fn get_string_from_request(mut request: Request) -> (String, Request) {
    let mut buf = Vec::new();
    request.as_reader().read_to_end(&mut buf).expect("unwrap this pls ");
    let body = std::str::from_utf8(&buf).unwrap().to_string();
    return (body, request);
}

fn respond_with_file(request: Request, file: &str, content_type: &str, redirect: bool) {
    let response = Response::from_file(File::open(&Path::new(file)).unwrap())
    .with_header(Header::from_bytes("Content-Type", content_type)
    .expect("That we had no proble unwrapp headers"));
    // eprintln!("{:?}", request.headers());
    request.respond(response).expect("Did send request (file)");
}

fn respond_with_string(request: Request, message: &str, content_type: &str, redirect: bool) {
    let mut json =  String::from("{\"message\" : \"");
    json.push_str(message);
    json.push_str("\"}");

    let response = Response::from_string(json).
    with_header(Header::from_bytes("Content-Type", content_type)
    .expect("That we had no proble unwrapp headers"));
    // eprintln!("{:?}", request.headers());
    request.respond(response).expect("Did send request (string)");
}

fn handle_query(request: Request, results:&Vec<(String, f64)>, content_type: &str) {
    let mut json =  String::from("{\"paths\" : [");
    for i in 0..results.len()-1 {
        json.push('"');
        let result = &results[i].0;
        json.push_str(result);
        json.push_str("\",");
    }
    json.push('"');
    json.push_str(&results[results.len()-1].0);
    json.push_str("\"]}");
    // eprintln!("{}", json);

    let response = Response::from_string(&json)
    .with_header(Header::from_bytes("Content-Type", content_type)
    .expect("That we had no proble unwrapp headers"));

    request.respond(response).expect("Did send request (file)");

} 

