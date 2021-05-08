use gateway_core::gateway::publisher::Channel;
use local::ble_connectivity::ble_client;
use local::types::config::Config;

use std::fs::File;
use std::sync::{Arc, Mutex};

fn main() -> () {
    //read configuration file
    let config: Config = serde_json::from_reader(File::open("config.json").unwrap()).unwrap();

    println!("Starting....");

    let channel = Arc::new(Mutex::new(Channel::new(
        config.node,
        config.local_pow,
        None,
    )));
    let (addr, msg) = match channel.lock().expect("").open() {
        Ok(a) => a,
        Err(_) => panic!("Could not connect to IOTA Node, try with another node!"),
    };
    println!("Channel root: {:?}", format!("{}:{}", addr, msg));
    println!(
        "\n To read the messages copy the channel root into https://explorer.iot2tangle.io/ \n "
    );

    ble_client::start(config.device_ids, config.reading_interval, channel)
}
