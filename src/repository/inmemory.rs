use std::cell::RefCell;

use crate::device::{Device, MacAddress};
use crate::repository::{DeleteError, DeviceRepository, InsertError};

pub struct InMemoryDeviceRepository {
    devices: RefCell<Vec<Device>>,
}

impl InMemoryDeviceRepository {
    pub fn new() -> Self {
        Self {
            devices: RefCell::new(vec![]),
        }
    }
}

impl DeviceRepository for InMemoryDeviceRepository {
    fn insert(&self, device: Device) -> Result<(), InsertError> {
        if self.devices.borrow().iter().any(|d| d.name == device.name) {
            Err(InsertError::Conflict)
        } else {
            self.devices.borrow_mut().push(device);
            Ok(())
        }
    }

    fn fetch_by_name(&self, name: &str) -> Option<Device> {
        self.devices
            .borrow()
            .iter()
            .find(|d| d.name == name)
            .cloned()
    }

    fn fetch_by_mac(&self, mac: &MacAddress) -> Option<Device> {
        self.devices
            .borrow()
            .iter()
            .find(|d| d.mac == *mac)
            .cloned()
    }

    fn delete(&self, name: &str) -> Result<(), DeleteError> {
        let mut result = Err(DeleteError::NotFound);

        self.devices.borrow_mut().retain(|d| {
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
        if self.devices.borrow().is_empty() {
            None
        } else {
            Some(self.devices.borrow().clone())
        }
    }
}
