pub mod file;
pub mod inmemory;

use std::sync::Arc;

use crate::device::{Device, DeviceName, MacAddress};

pub enum InsertError {
    Conflict,
    Other,
}

pub enum DeleteError {
    NotFound,
    Other,
}

impl std::fmt::Display for InsertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InsertError::Conflict => write!(f, "Device already exists in repository"),
            InsertError::Other => write!(f, "Unknown error while adding device to repository"),
        }
    }
}

impl std::fmt::Display for DeleteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeleteError::NotFound => write!(f, "Device not found in repository"),
            DeleteError::Other => write!(f, "Unknown error while deleting device from repository"),
        }
    }
}

pub trait DeviceRepository {
    fn insert(&self, device: Device) -> Result<(), InsertError>;
    fn delete(&self, name: &DeviceName) -> Result<(), DeleteError>;
    fn fetch_by_name(&self, name: &DeviceName) -> Option<Device>;
    fn fetch_by_mac(&self, mac: &MacAddress) -> Option<Device>;
    fn fetch_all(&self) -> Option<Vec<Device>>;
}

pub type SharedDeviceRepository = Arc<dyn DeviceRepository + Send + Sync>;
