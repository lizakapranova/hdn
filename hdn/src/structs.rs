use serde::{Deserialize, Serialize};

pub enum LogStatus {
    Connection,
    Load,
    Store,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Request {
    pub request_type: String,
    pub key: Option<String>,
    pub hash: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub response_status: String,
    pub requested_key: Option<String>,
    pub requested_hash: Option<String>,
}

impl Response {
    pub fn serialize(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }
}
