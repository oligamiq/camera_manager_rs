use crate::camera::IdentInformation;
use anyhow::Result;

pub mod nusb;

pub trait UsbBackend {
  fn get_usb_info(&self) -> Result<Vec<(usize, IdentInformation)>>;
}
