use crate::{
    device::{Device, MacAddress},
    repository::DeviceRepository,
    wol,
};

pub fn wake_device(mac: &str) {
    match MacAddress::try_from(mac) {
        Ok(mac) => match wol::wake(&mac) {
            Ok(_) => println!("Magic packet sent successfully"),
            Err(e) => eprintln!("{}", e),
        },
        Err(e) => eprintln!("{}", e),
    }
}

pub fn add_device(repo: &dyn DeviceRepository, name: &str, mac: &str) {
    match Device::try_from((name, mac)) {
        Ok(device) => match repo.insert(device) {
            Ok(_) => println!("Device ({}, {}) added", name, mac),
            Err(e) => eprintln!("{}", e),
        },
        Err(e) => eprintln!("{}", e),
    }
}

pub fn delete_device(repo: &dyn DeviceRepository, name: &str) {
    match repo.delete(name) {
        Ok(_) => println!("Device {} deleted", name),
        Err(e) => eprintln!("{}", e),
    }
}

pub fn show_devices(repo: &dyn DeviceRepository) {
    if let Some(devices) = repo.fetch_all() {
        for device in devices {
            println!("Device {} has mac address {}", device.name, device.mac);
        }
    }
}
