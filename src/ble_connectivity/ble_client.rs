extern crate blurz;

use crate::ble_connectivity::handlers::handle_sensor_data;
use crate::timestamp_in_sec;
use crate::types::sensor_data::SensorData;
use crate::types::sensor_type::SensorType;

use blurz::bluetooth_device::BluetoothDevice;
use blurz::bluetooth_gatt_characteristic::BluetoothGATTCharacteristic;
use blurz::bluetooth_gatt_service::BluetoothGATTService;
use blurz::bluetooth_session::BluetoothSession;
use gateway_core::gateway::publisher::Channel;
use std::str;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

///
/// Starts the server on the provided port, the server will hand over requests to the handler functions
///
pub fn start(device_list: Vec<String>, interval: u64, channel: Arc<Mutex<Channel>>) -> () {
    let session = &BluetoothSession::create_session(None).unwrap();

    //Main event loop
    loop {
        for device in &device_list {
            let device_path = format!("/org/bluez/hci0/dev_{}", device.replace(":", "_"));

            let device = BluetoothDevice::new(session, device_path);
            if let Err(_) = device.connect(30000) {
                println!("Failed to connect, trying again in next round....");
                continue;
            }

            let mut data_package = SensorData {
                iot2tangle: vec![],
                device: device.get_name().unwrap().clone(),
                timestamp: serde_json::to_value(timestamp_in_sec()).unwrap(),
            };
            let services_list = device.get_gatt_services().unwrap();

            for service_path in services_list {
                let service = BluetoothGATTService::new(session, service_path.to_string());
                let uuid_service = service.get_uuid().unwrap();

                if uuid_service.starts_with("00055000-0000-0000-") {
                    let mut characteristics = service.get_gatt_characteristics().unwrap();
                    characteristics.reverse();

                    if characteristics.len() < 1usize {
                        continue;
                    }

                    let first_value =
                        BluetoothGATTCharacteristic::new(session, characteristics[0].clone())
                            .read_value(None)
                            .unwrap();

                    let service_name = str::from_utf8(&first_value).unwrap().replace("\\", "");

                    let sensor_obj: serde_json::Value =
                        serde_json::from_str(&service_name).unwrap();
                    let name = sensor_obj["Name"].as_str().unwrap().to_string();

                    let mut sensor_type = SensorType {
                        sensor: name,
                        data: vec![],
                    };

                    for characteristic_path in characteristics.split_off(1) {
                        let characteristic =
                            BluetoothGATTCharacteristic::new(session, characteristic_path);
                        let uuid_char = characteristic.get_uuid().unwrap();

                        if uuid_char.starts_with("00055000-0000-0000-") {
                            let value = characteristic.read_value(None).unwrap();
                            let str_value = str::from_utf8(&value).unwrap();
                            let data: serde_json::Value = serde_json::from_str(str_value).unwrap();
                            sensor_type.data.push(data);
                        }
                    }
                    data_package.iot2tangle.push(sensor_type);
                }
            }

            let data_string = serde_json::to_string(&data_package).unwrap();

            handle_sensor_data(data_string, &channel);
        }
        thread::sleep(Duration::from_secs(interval));
    }
}
