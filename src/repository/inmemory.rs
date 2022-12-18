use std::sync::{Arc, Mutex};

use crate::{
    device::DeviceName,
    repository::{DeleteError, DeviceRepository, InsertError},
};
use crate::{
    device::{Device, MacAddress},
    repository::SharedDeviceRepository,
};

pub struct InMemoryDeviceRepository {
    devices: Mutex<Vec<Device>>,
}

impl InMemoryDeviceRepository {
    pub fn new() -> Self {
        Self {
            devices: Mutex::new(Vec::new()),
        }
    }

    pub fn new_shared() -> SharedDeviceRepository {
        Arc::new(Self::new())
    }
}

impl From<Vec<Device>> for InMemoryDeviceRepository {
    fn from(devices: Vec<Device>) -> Self {
        Self {
            devices: Mutex::new(devices),
        }
    }
}

impl DeviceRepository for InMemoryDeviceRepository {
    fn insert(&self, device: Device) -> Result<(), InsertError> {
        if self
            .devices
            .lock()
            .expect("lock")
            .iter()
            .any(|d| d.name == device.name)
        {
            Err(InsertError::Conflict)
        } else {
            self.devices.lock().expect("lock").push(device);
            Ok(())
        }
    }

    fn fetch_by_name(&self, name: &DeviceName) -> Option<Device> {
        self.devices
            .lock()
            .expect("lock")
            .iter()
            .find(|d| d.name == *name)
            .cloned()
    }

    fn fetch_by_mac(&self, mac: &MacAddress) -> Option<Device> {
        self.devices
            .lock()
            .expect("lock")
            .iter()
            .find(|d| d.mac == *mac)
            .cloned()
    }

    fn delete(&self, name: &DeviceName) -> Result<(), DeleteError> {
        let mut result = Err(DeleteError::NotFound);

        self.devices.lock().expect("lock").retain(|d| {
            if d.name == *name {
                result = Ok(());
                false
            } else {
                true
            }
        });

        result
    }

    fn fetch_all(&self) -> Option<Vec<Device>> {
        if self.devices.lock().expect("lock").is_empty() {
            None
        } else {
            Some(self.devices.lock().expect("lock").clone())
        }
    }
}
