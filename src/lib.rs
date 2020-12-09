//!
//! Channel Library
//!
#![deny()]
//#![cfg_attr(not(debug_assertions), deny(warnings))]
#[macro_use]
extern crate lazy_static;
extern crate gateway_core;
pub mod ble_connectivity;
pub mod device_auth;
pub mod types;

use std::time::{SystemTime, UNIX_EPOCH};
fn timestamp_in_sec() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
