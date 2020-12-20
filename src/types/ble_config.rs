use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct BleConfig {
    pub device_ble_name: String,
    pub service_uuid: String,
    pub char_uuid: String,
    pub desc_uuid: String,
}
