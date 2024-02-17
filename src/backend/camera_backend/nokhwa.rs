use super::CameraBackend;
use anyhow::Result;
use nokhwa::utils::CameraInfo;

pub struct NokhwaCameraBackend;

impl NokhwaCameraBackend {
    pub fn new() -> Self {
        NokhwaCameraBackend
    }

    pub fn get_nokhwa_data() -> Result<Vec<CameraInfo>> {
        let (sender, receiver) = std::sync::mpsc::channel();

        nokhwa::nokhwa_initialize(move |_x| {
            let cameras = nokhwa::query(nokhwa::utils::ApiBackend::Auto);
            sender.send(cameras).expect("Failed to send nokhwa data");
        });

        Ok(receiver.recv()??)
    }
}

impl CameraBackend for NokhwaCameraBackend {
    fn get_camera_info(&self) -> Result<Vec<(usize, crate::camera::IdentInformation)>> {
        let nokhwa_data = NokhwaCameraBackend::get_nokhwa_data()?;
        let mut camera_info = Vec::new();
        for camera in nokhwa_data {
            #[cfg(target_os = "windows")]
            {
                let misc = camera.misc();

                // camera_info.push((
                //     camera.index(),
                //     crate::camera::IdentInformation::new(camera.vendor_id, camera.product_id),
                // ));
                println!("Camera: {:#?}", camera);
            }
        }
        Ok(camera_info)
    }
}
