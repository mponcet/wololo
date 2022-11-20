use crate::{
    device::{Device, MacAddress},
    repository::{DeleteError, DeviceRepository, InsertError},
    wol,
};

pub fn wake_device(mac: &str) {
    match MacAddress::try_from(mac) {
        Ok(mac) => match wol::wake(&mac) {
            Ok(_) => println!("Magic packet sent successfully"),
            Err(_) => eprintln!("Wake on lan failed"),
        },
        Err(_) => eprintln!("Wrong mac address format"),
    }
}

pub fn add_device(repo: &mut impl DeviceRepository, name: &str, mac: &str) {
    match Device::new(name, mac) {
        Ok(device) => match repo.insert(device) {
            Err(InsertError::Conflict) => eprintln!("Name or mac address already exists"),
            _ => println!("Device ({}, {}) added", name, mac),
        },
        Err(_) => eprintln!("Wrong device name or mac address format"),
    }
}

pub fn delete_device(repo: &mut impl DeviceRepository, name: &str) {
    match repo.delete(name) {
        Ok(_) => println!("Device {} deleted", name),
        Err(DeleteError::NotFound) => eprintln!("Device not found"),
    }
}

pub fn show_devices(repo: &impl DeviceRepository) {
    if let Some(devices) = repo.fetch_all() {
        for device in devices {
            println!("Device {} has mac address {}", device.name, device.mac);
        }
    }
}
