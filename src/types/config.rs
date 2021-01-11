use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub device_ids: Vec<String>,
    pub node: String,
    pub reading_interval: u64,
    pub mwm: u8,
    pub local_pow: bool,
}
