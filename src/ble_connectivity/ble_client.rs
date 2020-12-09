extern crate blurz;
extern crate regex;

use crate::ble_connectivity::handlers::handle_sensor_data;
use crate::device_auth::keystore::KeyManager;
use crate::types::static_topic::StaticTopic;
use gateway_core::gateway::publisher::Channel;

use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{process, thread};

use std::str;

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
    ble_device: String,
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
        println!(
            "Device: {:?} Name: {:?}",
            device_path,
            device.get_name().ok()
        );
    }

    let device = BluetoothDevice::new(session, ble_device);

    if let Err(e) = device.connect(30000) {
        println!("Failed to connect {:?}: {:?}", device.get_id(), e);
    } else {
        println!("Connected!");
        thread::sleep(Duration::from_millis(5000));
    }

    let services_list = device.get_gatt_services().unwrap();

    for service_path in services_list {
        let service = BluetoothGATTService::new(session, service_path.to_string());
        let uuid = service.get_uuid().unwrap();
        let assigned_number = RE
            .captures(&uuid)
            .unwrap()
            .get(1)
            .map_or("", |m| m.as_str());

        println!(
            "Service UUID: {:?} Assigned Number: 0x{:?}",
            uuid, assigned_number
        );

        let characteristics = service.get_gatt_characteristics().unwrap();
        for characteristic_path in characteristics {
            let characteristic = BluetoothGATTCharacteristic::new(session, characteristic_path);
            let uuid = characteristic.get_uuid().unwrap();
            let assigned_number = RE
                .captures(&uuid)
                .unwrap()
                .get(1)
                .map_or("", |m| m.as_str());
            let flags = characteristic.get_flags().unwrap();

            println!(
                " Characteristic Assigned Number: 0x{:?} Flags: {:?}",
                assigned_number, flags
            );

            let descriptors = characteristic.get_gatt_descriptors().unwrap();
            for descriptor_path in descriptors {
                let descriptor = BluetoothGATTDescriptor::new(session, descriptor_path);
                let uuid = descriptor.get_uuid().unwrap();
                let assigned_number = RE
                    .captures(&uuid)
                    .unwrap()
                    .get(1)
                    .map_or("", |m| m.as_str());
                let value = descriptor.read_value(None).unwrap();

                let value = match &assigned_number[4..] {
                    "2901" => str::from_utf8(&value).unwrap().to_string(),
                    _ => format!("{:?}", value),
                };

                println!(
                    "    Descriptor Assigned Number: 0x{:?} Read Value: {:?}",
                    assigned_number, value
                );
            }
        }
    }
}
