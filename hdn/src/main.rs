pub mod server;
pub mod structs;

use chrono::Utc;
use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

fn main() {
    let storage: HashMap<String, String> = HashMap::new();
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server is listening on port 3333");
    let stream_storage = Arc::new(Mutex::new(storage));
    for stream in listener.incoming() {
        match stream {
            Err(e) => {
                println!("Error: {e}");
            }
            Ok(stream) => {
                let time_date = Utc::now();
                print!("{} ", time_date.to_rfc3339());
                println!("New connection: {}", stream.peer_addr().unwrap());
                let storage_ref = Arc::clone(&stream_storage);
                let (key, hash) =
                    std::thread::spawn(move || -> (Option<String>, Option<String>) {
                        server::handle_connection(stream, storage_ref)
                    })
                    .join()
                    .unwrap();
                if key.is_some() && hash.is_some() {
                    (*stream_storage.lock().unwrap()).insert(key.unwrap(), hash.unwrap());
                }
            }
        }
    }
}
