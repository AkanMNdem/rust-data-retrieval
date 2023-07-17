use paho_mqtt as mqtt;

fn main() -> mqtt::Result<()> {
    println!("Creating new client...");

    let server_uri = "ssl://nam1.cloud.thethings.network:8883";
    let username = "uva-engineers-way-sensors@ttn";
    let password = "NNSXS.H74NN25HX2EFPPAH5JMMZXJBDRQP3KIBMOIF3RI.FLFWZZDQPJVUA4Z2FKZG6RRQNJC37I2B5BTZ464YY5P5OG2M47EQ";
    let subscribe_topic = "v3/uva-engineers-way-sensors@ttn/devices/+/up";

    println!("Server URI: {}", server_uri);
    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("Subscribe Topic: {}", subscribe_topic);

    let client = mqtt::AsyncClient::new(server_uri)?;

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .server_uris(&[server_uri])
        .keep_alive_interval(std::time::Duration::from_secs(30))
        .clean_session(true)
        .user_name(username)
        .password(password)
        .finalize();

    println!("Connecting to the MQTT broker...");
    client.connect(conn_opts)?;

    println!("Successfully connected to the broker!");
    println!("Subscribing to the topic: {}", subscribe_topic);

    client.subscribe(subscribe_topic, 0)?;

    let stream = client.start_consuming();
    while let Ok(Some(message)) = stream.try_recv() {
        println!("Received message: {:?}", message.payload_str());
    }

    Ok(client.disconnect(None)?)
}
