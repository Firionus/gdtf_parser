#![cfg(test)]

use crate::GDTF;
use crate::utils::units::data_version::DataVersion;

pub mod fixture_type;
#[allow(dead_code)]
pub fn expect() -> GDTF {
    GDTF {
        data_version: DataVersion::Version1_1,
        fixture_type: fixture_type::expect(),
    }
}