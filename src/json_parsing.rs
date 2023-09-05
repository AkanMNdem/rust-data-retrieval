// src/parser.rs

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub temperature_f: Option<f32>,
    pub pressure_bar: Option<f32>,
    pub lorawan_airtime: Option<u8>,
    pub _meta: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub device_id: u64,
    pub received_timestamp: String,
    pub gateway_id: String,
    pub receiver: String,
}

pub fn parse_payload(json_str: &str) -> Result<Payload, serde_json::Error> {
    serde_json::from_str(json_str)
}
