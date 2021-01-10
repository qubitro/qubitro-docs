/* 
 * Rust example connecting to the Qubitro cloud via MQTT over TLS
 * 
 * Written by Samuel Archibald (@IoTPanic) and is owned and
 * maintained by Qubitro, Inc. github.com/qubitro/qubitro-docs-mqtt
 * 
 * 
 * Make sure to read the example Cargo.toml for dependencies
 */ 

use paho_mqtt as mqtt;
use std::process;
use std::time::Duration;
use std::thread;
use futures::executor::block_on;


fn main() {
    let host = "ssl://broker.qubitro.com:8883";
    let device_id = "PASTE_DEVICE_ID";
    let device_token = "PASTE_DEVICE_TOKEN";

    println!("Connecting to host {:? }...", host);

    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .persistence(mqtt::PersistenceType::None)
        .client_id(device_id)
        .finalize();

    let cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|err| {
        println!("Error creating the client: {}", err);
        process::exit(1);
    });

    if let Err(err) = block_on(async {

       let conn_opts = mqtt::ConnectOptionsBuilder::new()
           .user_name(device_id)
           .password(device_token)
           .ssl_options(mqtt::ssl_options::SslOptionsBuilder::new().finalize())
           .finalize();

        cli.connect(conn_opts).await?;

        loop{
           println!("Publishing a message on the topic: {}",device_id);
           let msg = mqtt::Message::new(device_id, "{\"Temperature\": 32}", mqtt::QOS_1);
           cli.publish(msg).await?;
           thread::sleep(Duration::from_secs(2))
        }
 
        Ok::<(), mqtt::Error>(())

    }) {
        eprintln!("Error {}", err);
    }
}