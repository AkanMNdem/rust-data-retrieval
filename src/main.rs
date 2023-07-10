use paho_mqtt as mqtt;

fn main() {
    println!("Creating new client...");
    let client = mqtt::AsyncClient::new("ssl://nam1.cloud.thethings.network:8883").unwrap();

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .server_uris(&["ssl://nam1.cloud.thethings.network:8883"])
        .keep_alive_interval(std::time::Duration::from_secs(30))
        .clean_session(true)
        .user_name("uva-engineers-way-sensors@ttn")
        .password("ZXDUEUJ7XISNZNAE4RR3ZMNJX4XXL66LS5VJXOA")
        .finalize();

    println!("Connecting to the MQTT broker...");
    let res = client.connect(conn_opts).wait();

    match res {
        Ok(_) => println!("Successfully connected to the broker!"),
        Err(e) => {
            println!("Failed to connect to the broker. {:?}", e);
            return;
        }
    }

    let subscribe_topic = "v3/uva-engineers-way-sensors@ttn/devices/+/up";
    println!("Subscribing to the topic: {}", subscribe_topic);
    client.subscribe(subscribe_topic, 1).wait().unwrap();

    let stream = client.start_consuming();
    while let Ok(Some(message)) = stream.try_recv() {
        // Handle the received message
        println!("Received message: {:?}", message.payload_str());
        // Process the message payload and extract the relevant data
    }

    let _ = client.disconnect(None);
}
