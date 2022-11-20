use crate::{
    cli,
    repository::{inmemory::InMemoryDeviceRepository, DeviceRepository},
};

#[test]
fn test_cli() {
    let repo = &mut InMemoryDeviceRepository::new();
    cli::add_device(repo, "pc1", "zz:01:02:03:04:05");
    assert!(repo.fetch_all().is_none());
    cli::add_device(repo, "pc1", "00:01:02:03:04:05");
    assert_eq!(repo.fetch_all().unwrap().len(), 1);
    cli::show_devices(repo);
    cli::delete_device(repo, "pc1");
    assert!(repo.fetch_all().is_none());
    cli::show_devices(repo);
}
