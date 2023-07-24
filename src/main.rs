//use openssl::ssl::{SslMethod, SslConnector};
use paho_mqtt as mqtt;

fn main() {
    println!("Creating new client...");

    let server_uri = "mqtts://nam1.cloud.thethings.network:8883";
    let username = "uva-engineers-way-sensors";
    let password = "NNSXS.H74NN25HX2EFPPAH5JMMZXJBDRQP3KIBMOIF3RI.FLFWZZDQPJVUA4Z2FKZG6RRQNJC37I2B5BTZ464YY5P5OG2M47EQ";
    let subscribe_topic = "v3/uva-engineers-way-sensors@ttn/devices/+/up";

    println!("Server URI: {}", server_uri);
    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("Subscribe Topic: {}", subscribe_topic);

    let ssl_opts = mqtt::SslOptionsBuilder::new()
        .trust_store("/Users/akanndem/.cargo/git/checkouts/paho.mqtt.rust-44b07fd42c95bba6/058c4f8/paho-mqtt-sys/paho.mqtt.c/test/ssl/capath/test-root-ca.pem")
        .expect("failed to set trust store")// Replace with your trust store path
        .key_store("/Users/akanndem/.cargo/git/checkouts/paho.mqtt.rust-44b07fd42c95bba6/058c4f8/paho-mqtt-sys/paho.mqtt.c/test/ssl/client.pem")// Replace with your key store path
        .expect("failed to set key store")
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

    let sub_token = client.subscribe(subscribe_topic, 0);
    if let Err(e) = sub_token.wait() {
        panic!("Failed to subscribe to the topic: {:?}", e);
    }

    let stream = client.start_consuming();
    let start_time = std::time::Instant::now();
    let max_duration = std::time::Duration::from_secs(600); // Exit after 10 minutes

    loop {
        if std::time::Instant::now() - start_time >= max_duration {
            break;
        }

        match stream.try_recv() {
            Ok(Some(message)) => {
                println!("Received message: {:?}", message.payload_str());
            }
            Ok(None) => {
                // No messages available; sleep for a while before trying again
                std::thread::sleep(std::time::Duration::from_secs(60)); // Sleep for 1 minute
            }
            Err(e) => {
                // An error occurred; print it and sleep for a while before trying again
                println!("Error: {:?}", e);
                std::thread::sleep(std::time::Duration::from_secs(60)); // Sleep for 1 minute
            }
        }
    }
}
