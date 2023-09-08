use serde::{Serialize, Deserialize};

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
    decoded_payload: DecodedPayload,
    #[serde(rename = "rx_metadata")]
    rx_metadata: Vec<RxMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DecodedPayload {
    #[serde(rename = "temperature_celsius")]
    temperature_celsius: Option<f32>,
    #[serde(rename = "humidity_percent")]
    humidity_percent: Option<f32>,
    // Add other fields as needed
}

#[derive(Debug, Serialize, Deserialize)]
struct RxMetadata {
    #[serde(rename = "gateway_ids")]
    gateway_ids: GatewayIDs,
}

#[derive(Debug, Serialize, Deserialize)]
struct GatewayIDs {
    #[serde(rename = "gateway_id")]
    gateway_id: String,
}

pub fn parse_payload(json_str: &str) -> Result<TTNPayload, serde_json::Error> {
    serde_json::from_str(json_str)
}
