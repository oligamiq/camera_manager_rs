use std::sync::Arc;

use crate::backend::{camera_backend::CameraBackend, usb_backend::UsbBackend};

#[derive(Clone, Debug)]
pub struct Camera<C: CameraBackend, U: UsbBackend> {
    pub(crate) port: u8,
    pub(crate) index: usize,
    pub(crate) info: IdentInformation,
    camera_backend: Arc<C>,
    usb_backend: Arc<U>,
}

#[derive(Clone, Debug)]
pub struct IdentInformation {
    vendor: u16,
    product: u16,
    #[cfg(not(target_os = "windows"))]
    serial_number: String,
}

impl IdentInformation {
    #[cfg(not(target_os = "windows"))]
    pub fn new(vendor: u16, product: u16, serial_number: String) -> Self {
        IdentInformation {
            vendor,
            product,
            serial_number,
        }
    }

    #[cfg(target_os = "windows")]
    pub fn new(vendor: u16, product: u16) -> Self {
        IdentInformation { vendor, product }
    }

    pub fn get_friendly_name(&self) -> String {
        usb_ids::Device::from_vid_pid(self.vendor, self.product)
            .map(|d| d.name())
            .unwrap_or(&"Unknown")
            .to_string()
    }
}
