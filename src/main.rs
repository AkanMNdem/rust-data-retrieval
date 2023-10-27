mod json_parsing;
use paho_mqtt as mqtt;
use serde_json::Value;

fn main() {
    println!("Creating new client...");

    let server_uri = "mqtts://nam1.cloud.thethings.network:8883";
    let username = "uva-engineers-way-sensors";
    let password = "NNSXS.H74NN25HX2EFPPAH5JMMZXJBDRQP3KIBMOIF3RI.FLFWZZDQPJVUA4Z2FKZG6RRQNJC37I2B5BTZ464YY5P5OG2M47EQ";
    let subscribe_topic = "v3/uva-engineers-way-sensors@ttn/devices/+/up";

    let ssl_opts = mqtt::SslOptionsBuilder::new()
        .trust_store("/Users/akanndem/Downloads/letsencrypt_truststore.pem") // Path to the combined trust store
        .expect("failed to set trust store")
        .finalize();

    let client = mqtt::AsyncClient::new(server_uri).expect("Failed to create client");

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(std::time::Duration::from_secs(30))
        .clean_session(true)
        .user_name(username)
        .password(password)
        .ssl_options(ssl_opts)
        .finalize();

    println!("Connecting to the MQTT broker...");
    let conn_token = client.connect(conn_opts);
    if let Err(e) = conn_token.wait() {
        panic!("Failed to connect to the broker: {:?}", e);
    }

    println!("Successfully connected to the broker!");
    println!("Subscribing to the topic: {}", subscribe_topic);
    let stream = client.start_consuming();
    let sub_token = client.subscribe(subscribe_topic, 0);
    if let Err(e) = sub_token.wait() {
        panic!("Failed to subscribe to the topic: {:?}", e);
    }


    match stream.recv() {
        Ok(Some(message)) => {
            let payload = message.payload_str();
            println!("Received message: {:?}", payload);

            // Use the parser to extract the desired data structure.
            match json_parsing::parse_payload(&payload) {
                Ok(parsed_data) => {
                    println!("Parsed Data: {:?}", parsed_data);

                },
                Err(e) => {
                    println!("Failed to parse the payload: {:?}", e);
                }
            }
        }
        Ok(None) => {
            println!("No message received.");
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
