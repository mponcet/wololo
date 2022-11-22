pub mod inmemory;

use crate::device::{Device, MacAddress};

pub enum InsertError {
    Conflict,
}

pub enum DeleteError {
    NotFound,
}

pub trait DeviceRepository {
    fn insert(&self, device: Device) -> Result<(), InsertError>;
    fn delete(&self, name: &str) -> Result<(), DeleteError>;
    fn fetch_by_name(&self, name: &str) -> Option<Device>;
    fn fetch_by_mac(&self, mac: &MacAddress) -> Option<Device>;
    fn fetch_all(&self) -> Option<Vec<Device>>;
}
