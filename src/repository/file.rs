use std::{io::Write, sync::Mutex};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tempfile::NamedTempFile;

use crate::{
    device::{Device, MacAddress},
    repository::{DeleteError, DeviceRepository, InsertError, SharedDeviceRepository},
};

#[derive(Debug)]
pub enum FileRepositoryError {
    DeserializeError,
    FileError,
}

pub struct FileRepository {
    path: PathBuf,
    devices: Mutex<Vec<Device>>,
}

impl FileRepository {
    pub fn try_new<P: AsRef<Path>>(path: P) -> Result<Self, FileRepositoryError> {
        let yaml = &std::fs::read_to_string(&path).map_err(|_| FileRepositoryError::FileError)?;
        let devices: Vec<_> =
            serde_yaml::from_str(yaml).map_err(|_| FileRepositoryError::DeserializeError)?;

        Ok(Self {
            path: path.as_ref().to_owned(),
            devices: Mutex::new(devices),
        })
    }

    pub fn try_new_shared<P: AsRef<Path>>(
        path: P,
    ) -> Result<SharedDeviceRepository, FileRepositoryError> {
        Ok(Arc::new(Self::try_new(path)?))
    }

    fn flush(&self, devices: &[Device]) -> Result<(), std::io::Error> {
        let mut tmpfile = NamedTempFile::new_in(".")?;
        writeln!(tmpfile, "{}", serde_yaml::to_string(devices).unwrap())?;
        tmpfile.into_temp_path().persist(self.path.as_path())?;

        Ok(())
    }
}

impl DeviceRepository for FileRepository {
    fn insert(&self, device: Device) -> Result<(), InsertError> {
        let mut devices = self.devices.lock().expect("lock");

        if devices.iter().any(|d| d.name == device.name) {
            Err(InsertError::Conflict)
        } else {
            devices.push(device);
            if let Err(err) = self.flush(&devices) {
                eprintln!("writeback failed: {}", err);
                devices.pop();
                return Err(InsertError::Other);
            }
            Ok(())
        }
    }

    fn delete(&self, name: &str) -> Result<(), DeleteError> {
        let mut devices = self.devices.lock().expect("lock");

        if let Some(index) = devices.iter().position(|d| d.name == name) {
            let deleted = devices[index].clone();
            devices.remove(index);

            if let Err(err) = self.flush(&devices) {
                eprintln!("writeback failed: {}", err);
                devices.push(deleted);
                return Err(DeleteError::Other);
            }
            Ok(())
        } else {
            Err(DeleteError::NotFound)
        }
    }

    fn fetch_by_name(&self, name: &str) -> Option<Device> {
        self.devices
            .lock()
            .expect("lock")
            .iter()
            .find(|d| d.name == name)
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

    fn fetch_all(&self) -> Option<Vec<Device>> {
        let devices = self.devices.lock().expect("lock");
        if devices.is_empty() {
            None
        } else {
            Some(devices.clone())
        }
    }
}
