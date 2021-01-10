/* 
 * Rust example connecting to the Qubitro cloud via MQTT over TLS
 * 
 * Written by Samuel Archibald (@IoTPanic) and is owned and
 * maintained by Qubitro, Inc. github.com/qubitro/qubitro-docs-mqtt
 * 
 * 
 * Make sure to read the example Cargo.toml for dependencies
 */ 

use std::{
    env,
    process,
    time::Duration,
    thread
};
use futures::{
    executor::block_on,
    stream::StreamExt,
};
use paho_mqtt as mqtt;

const TOPICS: &[&str] = &[ "test", "hello" ];
const QOS: &[i32] = &[1, 1];

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
 
     let mut cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|err| {
         println!("Error creating the client: {}", err);
         process::exit(1);
     });
 
     if let Err(err) = block_on(async {

              // Get message stream before connecting.
              let mut strm = cli.get_stream(25);

              // Define the set of options for the connection
              let lwt = mqtt::Message::new("test", "Async subscriber lost connection",
                                           mqtt::QOS_1);
      

        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .user_name(device_id)
            .password(device_token)
            .ssl_options(mqtt::ssl_options::SslOptionsBuilder::new().finalize())
            .clean_session(false)
            .finalize();
 
        println!("Connecting to Qubitro...");
        cli.connect(conn_opts).await?;

        println!("Subscribing to topics: {:?}", TOPICS);
        cli.subscribe_many(TOPICS, QOS).await?;

        // Just loop on incoming messages.
        println!("Waiting for messages...");

        while let Some(msg_opt) = strm.next().await {
            if let Some(msg) = msg_opt {
                println!("{}", msg);
            }
            else {
                // A "None" means we were disconnected. Try to reconnect...
                println!("Lost connection. Attempting reconnect.");
                while let Err(err) = cli.reconnect().await {
                    println!("Error reconnecting: {}", err);
                    // For tokio use: tokio::time::delay_for()
                    thread::sleep(Duration::from_secs(2))
                }
            }
        }
  
         Ok::<(), mqtt::Error>(())

     }) {
         eprintln!("Error {}", err);
     }
 }