use paho_mqtt as mqtt;

fn main() {
    println!("Creating new client...");

    let server_uri = "ssl://nam1.cloud.thethings.network:8883";
    let username = "uva-engineers-way-sensors@ttn";
    let password = "NNSXS.H74NN25HX2EFPPAH5JMMZXJBDRQP3KIBMOIF3RI.FLFWZZDQPJVUA4Z2FKZG6RRQNJC37I2B5BTZ464YY5P5OG2M47EQ";
    let subscribe_topic = "v3/uva-engineers-way-sensors@ttn/devices/+/up";

    println!("Server URI: {}", server_uri);
    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("Subscribe Topic: {}", subscribe_topic);

    let client = mqtt::AsyncClient::new(server_uri).expect("Failed to create client");

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(std::time::Duration::from_secs(30))
        .clean_session(true)
        .user_name(username)
        .password(password)
        .finalize();

    println!("Connecting to the MQTT broker...");
    let conn_token = client.connect(conn_opts);
    if let Err(e) = conn_token.wait() {
        panic!("Failed to connect to the broker: {:?}", e);
    }

    println!("Successfully connected to the broker!");
    println!("Subscribing to the topic: {}", subscribe_topic);

    let sub_token = client.subscribe(subscribe_topic, 0);
    if let Err(e) = sub_token.wait() {
        panic!("Failed to subscribe to the topic: {:?}", e);
    }

    let stream = client.start_consuming();
    while let Ok(Some(message)) = stream.try_recv() {
        println!("Received message: {:?}", message.payload_str());
    }

    let disc_token = client.disconnect(None);
    if let Err(e) = disc_token.wait() {
        panic!("Failed to disconnect from the broker: {:?}", e);
    }
}
