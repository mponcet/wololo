use std::{
    path::{Path, PathBuf},
    result::Result,
};

use std::io::Write;
use tempfile::NamedTempFile;

use crate::{
    device::{Device, MacAddress},
    repository::{inmemory::InMemoryDeviceRepository, DeleteError, DeviceRepository, InsertError},
};

#[derive(Debug)]
pub enum FileRepositoryError {
    DeserializeError,
    FileError,
}

pub struct FileRepository {
    path: PathBuf,
    inmemory: InMemoryDeviceRepository,
}

impl FileRepository {
    pub fn try_new<P: AsRef<Path>>(path: P) -> Result<Self, FileRepositoryError> {
        let yaml = &std::fs::read_to_string(&path).map_err(|_| FileRepositoryError::FileError)?;
        let devices: Vec<_> =
            serde_yaml::from_str(yaml).map_err(|_| FileRepositoryError::DeserializeError)?;

        Ok(Self {
            path: path.as_ref().to_owned(),
            inmemory: InMemoryDeviceRepository::from(devices),
        })
    }

    fn flush(&self) -> Result<(), std::io::Error> {
        let mut tmpfile = NamedTempFile::new_in(".")?;
        let yaml = serde_yaml::to_string(&self.inmemory.fetch_all()).unwrap();
        writeln!(tmpfile, "{}", yaml)?;
        tmpfile.into_temp_path().persist(&self.path)?;

        Ok(())
    }
}

impl DeviceRepository for FileRepository {
    fn insert(&self, device: Device) -> Result<(), InsertError> {
        self.inmemory
            .insert(device)
            .map(|_| self.flush().expect("writeback failed"))
    }

    fn delete(&self, name: &str) -> Result<(), DeleteError> {
        self.inmemory.delete(name)
    }

    fn fetch_by_name(&self, name: &str) -> Option<Device> {
        self.inmemory.fetch_by_name(name)
    }

    fn fetch_by_mac(&self, mac: &MacAddress) -> Option<Device> {
        self.inmemory.fetch_by_mac(mac)
    }

    fn fetch_all(&self) -> Option<Vec<Device>> {
        self.inmemory.fetch_all()
    }
}
