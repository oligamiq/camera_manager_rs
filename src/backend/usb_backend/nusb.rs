use super::UsbBackend;
use anyhow::Result;
use nusb::DeviceInfo;

pub struct NusbBackend;

impl NusbBackend {
    pub fn new() -> Self {
        NusbBackend
    }

    pub fn get_nusb_data() -> Result<Vec<DeviceInfo>> {
      Ok(nusb::list_devices()?
          .filter(|device| {
              device
                  .interfaces()
                  // https://www.usb.org/defined-class-codes
                  .fold(false, |be, interface| be || interface.class() == 0x0E)
          })
          .collect::<Vec<_>>())
    }
}

impl UsbBackend for NusbBackend {
    fn get_usb_info(&self) -> Result<Vec<(usize, crate::camera::IdentInformation)>> {
        let nusb_data = NusbBackend::get_nusb_data()?;
        let mut usb_info = Vec::new();
        for device in nusb_data {
            #[cfg(target_os = "windows")]
            {
                usb_info.push((
                    device.port_number() as usize,
                    crate::camera::IdentInformation::new(device.vendor_id(), device.product_id()),
                ));
            }
            println!("Device: {:#?}", device);
        }
        Ok(usb_info)
    }
}
