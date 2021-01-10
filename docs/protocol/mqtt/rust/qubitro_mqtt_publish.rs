/* 
 * Rust example connecting to the Qubitro cloud via MQTT over TLS
 * 
 * Written by Samuel Archibald (@IoTPanic) and is owned and
 * maintained by Qubitro, Inc. https://github.com/qubitro/qubitro-docs/tree/main/docs/protocol/mqtt/rust
 * 
 * Make sure to read the example Cargo.toml for dependencies
 */ 

use paho_mqtt as mqtt;
use std::process;
use futures::executor::block_on;

fn main() {
    let host = "ssl://broker.qubitro.com:8883";
    let dev_id = "PASTE_DEVICE_ID_HERE";
    let dev_token = "PASTE_DEVICE_TOKEN_HERE";

    println!("Connecting to host {:? }...", host);

    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .persistence(mqtt::PersistenceType::None)
        .client_id(dev_id)
        .finalize();

    let cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|err| {
        println!("Error creating the client: {}", err);
        process::exit(1);
    });

    if let Err(err) = block_on(async {
        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .password(dev_token)
            .user_name(dev_id)
            .ssl_options(mqtt::ssl_options::SslOptionsBuilder::new().finalize())
            .finalize();

        cli.connect(conn_opts).await?;

        // Create a message and publish it
        println!("Published");
        let msg = mqtt::Message::new(dev_id, "{\"Temperature\": 32}", mqtt::QOS_1);
        cli.publish(msg).await?;

        // Disconnect
        cli.disconnect(None).await?;

        Ok::<(), mqtt::Error>(())
    }) {
        eprintln!("Error {}", err);
    }
}