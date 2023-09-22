use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TTNPayload {
    #[serde(rename = "end_device_ids")]
    end_device_ids: DeviceIDs,
    #[serde(rename = "received_at")]
    received_at: String,
    #[serde(rename = "uplink_message")]
    uplink_message: UplinkMessage,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeviceIDs {
    #[serde(rename = "dev_eui")]
    dev_eui: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UplinkMessage {
    #[serde(rename = "decoded_payload")]
    decoded_payload: HashMap<String, Value>, // Use a HashMap for dynamic fields
    #[serde(rename = "rx_metadata")]
    rx_metadata: Vec<RxMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RxMetadata {
    #[serde(rename = "gateway_ids")]
    gateway_ids: GatewayIDs,
    rssi: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct GatewayIDs {
    #[serde(rename = "gateway_id")]
    gateway_id: String,
}

pub fn parse_payload(json_str: &str) -> Result<TTNPayload, serde_json::Error> {
    serde_json::from_str(json_str)
}

pub fn extract_metadata(payload: &TTNPayload) -> HashMap<String, Value> {
    let mut metadata = HashMap::new();

    // Finding the best RSSI gateway
    let best_rssi_metadata = payload.uplink_message.rx_metadata.iter()
        .max_by_key(|meta| meta.rssi);

    if let Some(best_rssi_meta) = best_rssi_metadata {
        metadata.insert("rssi".to_string(), Value::Number(best_rssi_meta.rssi.into()));
        metadata.insert("gateway_id".to_string(), Value::String(best_rssi_meta.gateway_ids.gateway_id.clone()));
    }

    metadata.insert("received_time".to_string(), Value::String(payload.received_at.clone()));
    metadata.insert("device_id".to_string(), Value::String(payload.end_device_ids.dev_eui.clone()));
    metadata.insert("receiver".to_string(), Value::String("http-ttn-mqtt".to_string()));

    metadata
}

// Function that helps extract dynamic fields from the uplink message
pub fn extract_numeric_fields(payload: &TTNPayload) -> HashMap<String, f64> {
    let mut numeric_fields = HashMap::new();
    for (key, value) in payload.uplink_message.decoded_payload.iter() {
        if let Value::Number(num) = value {
            if let Some(n) = num.as_f64() {
                numeric_fields.insert(key.clone(), n);
            }
        }
    }
    numeric_fields
}
