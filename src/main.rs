pub mod lex;
pub mod parse;
pub mod server;
pub mod hash;
pub mod engine;


const DIRECTORY: &'static str = "docs.gl";

fn main() {

    let all_hashed_documents = hash::hash_all_documents(DIRECTORY, true);
    // let x = get_local_directories(DIRECTORY);
    // for i in x {
    //     println!("{}",i);
    // }
    server::start_server(all_hashed_documents, DIRECTORY);
        
}

