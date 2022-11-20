use crate::device::{Device, MacAddress};
use crate::repository::{DeleteError, DeviceRepository, InsertError};

pub struct InMemoryDeviceRepository {
    devices: Vec<Device>,
}

impl InMemoryDeviceRepository {
    pub fn new() -> Self {
        Self { devices: vec![] }
    }
}

impl DeviceRepository for InMemoryDeviceRepository {
    fn insert(&mut self, device: Device) -> Result<(), InsertError> {
        if self.devices.iter().any(|d| d.name == device.name) {
            Err(InsertError::Conflict)
        } else {
            self.devices.push(device);
            Ok(())
        }
    }

    fn fetch_by_name(&self, name: &str) -> Option<Device> {
        self.devices.iter().find(|d| d.name == name).cloned()
    }

    fn fetch_by_mac(&self, mac: &MacAddress) -> Option<Device> {
        self.devices.iter().find(|d| d.mac == *mac).cloned()
    }

    fn delete(&mut self, name: &str) -> Result<(), DeleteError> {
        let mut result = Err(DeleteError::NotFound);

        self.devices.retain(|d| {
            if d.name == name {
                result = Ok(());
                false
            } else {
                true
            }
        });

        result
    }

    fn fetch_all(&self) -> Option<Vec<Device>> {
        if self.devices.is_empty() {
            None
        } else {
            Some(self.devices.clone())
        }
    }
}
