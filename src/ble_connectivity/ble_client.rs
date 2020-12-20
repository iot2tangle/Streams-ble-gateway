extern crate blurz;
extern crate regex;

use crate::ble_connectivity::handlers::handle_sensor_data;
use crate::device_auth::keystore::KeyManager;
use crate::types::ble_config::BleConfig;
use gateway_core::gateway::publisher::Channel;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use std::str;

extern crate btleplug;
extern crate rand;

#[cfg(target_os = "linux")]
use blurz::bluetooth_adapter::BluetoothAdapter;
use blurz::bluetooth_device::BluetoothDevice;
use blurz::bluetooth_discovery_session::BluetoothDiscoverySession;
use blurz::bluetooth_gatt_characteristic::BluetoothGATTCharacteristic;
use blurz::bluetooth_gatt_descriptor::BluetoothGATTDescriptor;
use blurz::bluetooth_gatt_service::BluetoothGATTService;
use blurz::bluetooth_session::BluetoothSession;
use regex::Regex;

const UUID_REGEX: &str = r"([0-9a-f]{8})-(?:[0-9a-f]{4}-){3}[0-9a-f]{12}";
lazy_static! {
    static ref RE: Regex = Regex::new(UUID_REGEX).unwrap();
}

///
/// Starts the server on the provided port, the server will hand over requests to the handler functions
///
pub async fn start(
    device_list: Vec<String>,
    ble_config: BleConfig,
    channel: Arc<Mutex<Channel>>,
    keystore: Arc<Mutex<KeyManager>>,
) -> () {
    let session = &BluetoothSession::create_session(None).unwrap();
    let adapter: BluetoothAdapter = BluetoothAdapter::init(session).unwrap();
    let adapter_id = adapter.get_id();
    let discover_session = BluetoothDiscoverySession::create_session(&session, adapter_id).unwrap();

    discover_session.start_discovery().unwrap();
    let devices = adapter.get_device_list().unwrap();
    discover_session.stop_discovery().unwrap();

    println!("Scanning..");
    for device_path in devices {
        let device = BluetoothDevice::new(session, device_path.to_string());
        println!("{:?}-{:?}", device.get_address(), device.get_name());
    }

    let device = BluetoothDevice::new(session, ble_config.device_ble_name);

    loop {
        if let Err(e) = device.connect(30000) {
            println!("Failed to connect, trying again....");
            println!("{}", e);
        } else {
            println!("Connected!");
            break;
        }
        thread::sleep(Duration::from_millis(8000));
    }
    loop {
        let services_list = device.get_gatt_services().unwrap();

        for service_path in services_list {
            let service = BluetoothGATTService::new(session, service_path.to_string());
            let uuid_service = service.get_uuid().unwrap();

            if uuid_service == ble_config.service_uuid {
                let characteristics = service.get_gatt_characteristics().unwrap();
                for characteristic_path in characteristics {
                    let characteristic =
                        BluetoothGATTCharacteristic::new(session, characteristic_path);
                    let uuid_char = characteristic.get_uuid().unwrap();

                    if uuid_char == ble_config.char_uuid {
                        let descriptors = characteristic.get_gatt_descriptors().unwrap();
                        for descriptor_path in descriptors {
                            let descriptor = BluetoothGATTDescriptor::new(session, descriptor_path);
                            let uuid_desc = descriptor.get_uuid().unwrap();
                            let value = descriptor.read_value(None).unwrap();

                            if uuid_desc == ble_config.desc_uuid {
                                println!("Value Sent {:?}", str::from_utf8(&value).unwrap());
                            }
                        }
                    }
                }
            }
        }
        thread::sleep(Duration::from_millis(10000));
    }
}
