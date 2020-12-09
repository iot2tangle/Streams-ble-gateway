use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub whitelisted_device_ids: Vec<String>,
    pub device_ble_name: String,
    pub node: String,
    pub mwm: u8,
    pub local_pow: bool,
}
