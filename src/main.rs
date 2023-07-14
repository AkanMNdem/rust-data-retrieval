use std::error::Error;

use paho_mqtt as mqtt;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Creating new client...");
    let client = mqtt::AsyncClient::new("tcp://nam1.cloud.thethings.network:8883")?;

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .server_uris(&["ssl://nam1.cloud.thethings.network:8883"])
        .keep_alive_interval(std::time::Duration::from_secs(30))
        .clean_session(true)
        .user_name("uva-engineers-way-sensors@ttn")
        .password("NNSXS.H74NN25HX2EFPPAH5JMMZXJBDRQP3KIBMOIF3RI.FLFWZZDQPJVUA4Z2FKZG6RRQNJC37I2B5BTZ464YY5P5OG2M47EQ")
        .finalize();

    println!("Connecting to the MQTT broker...");
    client.connect(conn_opts).wait()?;

    let subscribe_topic = "v3/uva-engineers-way-sensors@ttn/devices/+/up";
    println!("Subscribing to the topic: {}", subscribe_topic);
    client.subscribe(subscribe_topic, 0).wait()?;

    let stream = client.start_consuming();
    while let Ok(Some(message)) = stream.try_recv() {
        // Handle the received message
        println!("Received message: {:?}", message.payload_str());
        // Process the message payload and extract the relevant data
    }

    let _ = client.disconnect(None);
    Ok(())
}
