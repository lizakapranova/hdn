use std::collections::HashMap;
use std::io::{prelude::*, BufReader, Read};
use std::net::TcpStream;
use std::sync::{Arc, Mutex, MutexGuard};

use chrono::Utc;

use crate::structs::*;

fn size_printer(storage: MutexGuard<HashMap<String, String>>) {
    print!("Storage size: {}.", storage.len());
}
fn log_output(
    status: LogStatus,
    stream: &TcpStream,
    storage: MutexGuard<HashMap<String, String>>,
    data: Request,
) {
    print!("{} ", (*stream).peer_addr().unwrap());
    let time_date = Utc::now();
    print!("{} ", time_date.to_rfc3339());
    match status {
        LogStatus::Connection => {
            println!("New client {}", data.request_type);
            println!("Connection established.");
            size_printer(storage);
        }
        LogStatus::Load => {
            print!("Received request to write new value {}", data.hash.unwrap());
            println!("by key {}.", data.key.unwrap());
            size_printer(storage);
        }
        LogStatus::Store => {
            println!(
                "Received request to get value by key {}.",
                data.key.unwrap()
            );
            size_printer(storage);
        }
    }
}
fn stream_function(mut stream: TcpStream, ans: Response) {
    let ans_json = ans.serialize();
    stream.write_all(&ans_json.unwrap()).unwrap();
}
fn store(
    storage: MutexGuard<HashMap<String, String>>,
    deserialized_data: Request,
    stream: TcpStream,
    key: &mut Option<String>,
    hash: &mut Option<String>,
) {
    let output_data = deserialized_data.clone();
    *key = deserialized_data.key;
    *hash = deserialized_data.hash;
    // storage.insert(
    //     deserialized_data.key.unwrap(),
    //     deserialized_data.hash.unwrap(),
    // );
    let ans = Response {
        response_status: ("success".to_string()),
        requested_key: (None),
        requested_hash: (None),
    };
    log_output(LogStatus::Store, &stream, storage, output_data);
    stream_function(stream, ans);
}

fn load(
    storage: MutexGuard<HashMap<String, String>>,
    deserialized_data: Request,
    stream: TcpStream,
) {
    let mut ans = Response {
        response_status: ("none").to_string(),
        requested_key: None,
        requested_hash: None,
    };
    let load_data = deserialized_data.clone();
    let output_data = deserialized_data.clone();
    if storage.contains_key(&(deserialized_data.key.unwrap())) {
        ans = Response {
            response_status: ("success").to_string(),
            requested_key: load_data.key,
            requested_hash: Some(load_data.hash.unwrap()),
        };
    } else {
        ans.response_status = ("key not found").to_string();
    };
    log_output(LogStatus::Load, &stream, storage, output_data);
    stream_function(stream, ans);
}

pub fn handle_connection(
    mut stream: TcpStream,
    arc_storage: Arc<Mutex<HashMap<String, String>>>,
) -> (Option<String>, Option<String>) {
    let storage = arc_storage.lock().unwrap();

    let mut buffer = vec![];
    let request = BufReader::new(&mut stream).read(&mut buffer).unwrap();
    let deserialized_data: Request = serde_json::from_str(&request.to_string()).unwrap();

    let store_data = deserialized_data.clone();
    let load_data = deserialized_data.clone();

    let mut key: Option<String> = None;
    let mut hash: Option<String> = None;

    match deserialized_data.request_type.as_str() {
        "store" => store(storage, store_data, stream, &mut key, &mut hash),
        "load" => load(storage, load_data, stream),
        _ => log_output(LogStatus::Connection, &stream, storage, deserialized_data),
    }
    (key, hash)
}
