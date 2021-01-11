use crate::timestamp_in_sec;
use crate::types::sensor_data::SensorData;
use std::sync::{Arc, Mutex};

use gateway_core::gateway::publisher::Channel;

pub fn handle_sensor_data(data: String, channel: &Arc<Mutex<Channel>>) -> () {
    let data = data.to_owned();
    let json_data: serde_json::Result<SensorData> = serde_json::from_str(&data);
    match json_data {
        Ok(sensor_data) => {
            println!("New Data Recieved -- {:?}", timestamp_in_sec());
            let mut channel = channel.lock().unwrap();
            match channel.write_signed(&sensor_data) {
                Ok(_) => (),
                Err(_e) => {
                    println!("Error: Could not send data to Tangle, try switching nodes");
                    ()
                }
            };
        }
        Err(_e) => {
            println!(
                "New Message Recieved -- {:?} -- incorrectly formatted Data",
                timestamp_in_sec()
            );
        }
    }
    ()
}
