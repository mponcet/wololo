pub mod file;
pub mod inmemory;

use std::sync::Arc;

use crate::device::{Device, MacAddress};

pub enum InsertError {
    Conflict,
    Other,
}

pub enum DeleteError {
    NotFound,
    Other,
}

pub trait DeviceRepository {
    fn insert(&self, device: Device) -> Result<(), InsertError>;
    fn delete(&self, name: &str) -> Result<(), DeleteError>;
    fn fetch_by_name(&self, name: &str) -> Option<Device>;
    fn fetch_by_mac(&self, mac: &MacAddress) -> Option<Device>;
    fn fetch_all(&self) -> Option<Vec<Device>>;
}

pub type SharedDeviceRepository = Arc<dyn DeviceRepository + Send + Sync>;
