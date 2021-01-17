use blurz::bluetooth_adapter::BluetoothAdapter;
use blurz::bluetooth_device::BluetoothDevice;
use blurz::bluetooth_discovery_session::BluetoothDiscoverySession;
use blurz::bluetooth_session::BluetoothSession;

///
/// Scans and lists all Named Bluetooth devices in reach
///
fn main() -> () {
    let session = &BluetoothSession::create_session(None).unwrap();
    let adapter: BluetoothAdapter = BluetoothAdapter::init(session).unwrap();
    let adapter_id = adapter.get_id();
    let discover_session = BluetoothDiscoverySession::create_session(&session, adapter_id).unwrap();

    discover_session.start_discovery().unwrap();
    let devices = adapter.get_device_list().unwrap();
    discover_session.stop_discovery().unwrap();

    println!("Listing Surrounding Devices ...");
    for device_path in devices {
        let device = BluetoothDevice::new(session, device_path.to_string());
        if let Ok(n) = device.get_name() {
            println!(
                "MAC Address:{:?} - Name: {:?} ",
                device.get_address().unwrap(),
                n,
            );
        }
    }
}
