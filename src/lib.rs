use std::sync::Mutex;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tungstenite::Message;

pub static GLOBAL_STATUS: Lazy<Mutex<Status>> = Lazy::new(|| Mutex::new(Status::Inactive));

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Active,
    Sharing,
    Inactive,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct WsResponse {
    pub status: Status,
}

impl WsResponse {
    pub fn new(status: Status) -> Self {
        Self { status }
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    pub fn message(status: Status) -> Message {
        Message::Text(Self::new(status).to_json().try_into().unwrap())
    }
}

pub fn set_status(s: &Status) {
    let mut gs = GLOBAL_STATUS.lock().unwrap();
    *gs = s.clone();
}

pub fn get_brwoser_activity_status() -> Status {
    *GLOBAL_STATUS.lock().unwrap()
}
