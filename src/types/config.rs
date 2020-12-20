use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub whitelisted_device_ids: Vec<String>,
    pub device_ble_name: String,
    pub service_uuid: String,
    pub char_uuid: String,
    pub desc_uuid: String,
    pub node: String,
    pub mwm: u8,
    pub local_pow: bool,
}
