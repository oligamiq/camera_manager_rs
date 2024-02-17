use crate::camera::IdentInformation;
use anyhow::Result;

pub mod nokhwa;

pub trait CameraBackend {
    fn get_camera_info(&self) -> Result<Vec<(usize, IdentInformation)>>;
}
