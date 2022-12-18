use crate::repository::SharedDeviceRepository;

pub mod slack;

pub trait WakeOnLanService {
    fn run(
        &self,
        repo: SharedDeviceRepository,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
