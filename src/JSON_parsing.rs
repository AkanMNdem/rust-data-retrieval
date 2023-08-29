

use serde::{Deserialize, Serialize};
use serde_json::Value;

struct Data {
    temperature_F: u32,
    pressure_
}
// meta data struct will go here

pub fn parse_payload(payload: &str) -> Data {
    // parsing logic will go here
}
