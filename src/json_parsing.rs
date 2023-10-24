use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::Value;
use std::mem::size_of;

#[derive(Debug, Deserialize)]
pub struct TTNPayload {
    #[serde(rename = "end_device_ids")]
    end_device_ids: EndDeviceIds,

    #[serde(rename = "uplink_message")]
    uplink_message: UplinkMessage,
}

#[derive(Debug, Deserialize)]
pub struct EndDeviceIds {
    #[serde(rename = "dev_eui")]
    dev_eui: String,

    #[serde(rename = "application_ids")]
    application_ids: ApplicationIds,
}

#[derive(Debug, Deserialize)]
pub struct ApplicationIds {
    #[serde(rename = "application_id")]
    application_id: String,
}

#[derive(Debug, Deserialize)]
pub struct UplinkMessage {
    #[serde(rename = "f_cnt")]
    f_cnt: u32,

    // This field is not present in the provided payload, but I'm including it for completeness.
    // confirmed: bool,

    #[serde(rename = "settings")]
    settings: Settings,

    #[serde(rename = "consumed_airtime")]
    consumed_airtime: String,

    #[serde(rename = "rx_metadata")]
    rx_metadata: Vec<RxMetadata>,

    #[serde(rename = "received_at")]
    received_at: String,

    #[serde(rename = "decoded_payload")]
    decoded_payload: HashMap<String,Value>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    frequency: String,

    #[serde(rename = "data_rate")]
    data_rate: DataRate,

}

#[derive(Debug, Deserialize)]
pub struct DataRate {
    lora: Lora,
}

#[derive(Debug, Deserialize)]
pub struct Lora {
    #[serde(rename = "bandwidth")]
    bandwidth: u32,
    #[serde(rename = "spreading_factor")]
    spreading_factor: u32,
    #[serde(rename = "coding_rate")]
    coding_rate: String,
}

#[derive(Debug, Deserialize)]
pub struct RxMetadata {
    rssi: i32,
    snr: f32,

    #[serde(rename = "gateway_ids")]
    gateway_ids: GatewayIds,
}

#[derive(Debug, Deserialize)]
pub struct GatewayIds {
    #[serde(rename = "gateway_id")]
    gateway_id: String,
}


pub fn parse_payload(json_str: &str) -> Result<TTNPayload, serde_json::Error> {
    serde_json::from_str(json_str)
}


// pub fn extract_lora_data_points(payload: &TTNPayload) ->

// This function parses the metadata and stores it in a HashMap with a string for the field's
// name and its value. Is currently not used
pub fn extract_metadata(payload: &TTNPayload) -> HashMap<String, Value> {
    let mut metadata = HashMap::new();

    // Finding the best RSSI gateway
    let best_rssi_metadata = payload.uplink_message.rx_metadata.iter()
        .max_by_key(|meta| meta.rssi);

    if let Some(best_rssi_meta) = best_rssi_metadata {
        metadata.insert("rssi".to_string(), Value::Number(best_rssi_meta.rssi.into()));
        metadata.insert("gateway_id".to_string(), Value::String(best_rssi_meta.gateway_ids.gateway_id.clone()));
    }

    metadata.insert("received_time".to_string(), Value::String(payload.uplink_message.received_at.clone()));
    metadata.insert("device_id".to_string(), Value::String(payload.end_device_ids.dev_eui.clone()));
    metadata.insert("receiver".to_string(), Value::String("http-ttn-mqtt".to_string()));

    metadata
}

// Function that helps extract dynamic fields from the uplink message; Currently is not used
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
