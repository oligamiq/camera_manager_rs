use camera_manager::backend::{camera_backend::{nokhwa::NokhwaCameraBackend, CameraBackend as _}, usb_backend::{nusb::NusbBackend, UsbBackend as _}};

pub fn main() {
    NokhwaCameraBackend.get_camera_info().unwrap();
    NusbBackend.get_usb_info().unwrap();
}
