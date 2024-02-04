use anyhow::Result;
use rusb::{Device, UsbContext};

pub trait DeviceId {
    fn device_id(&self) -> Result<String>;
}

impl<T: UsbContext> DeviceId for Device<T> {
    fn device_id(&self) -> Result<String> {
        let device_desc = self.device_descriptor()?;
        let device_id = format!(
            "{:04x}:{:04x}",
            device_desc.vendor_id(),
            device_desc.product_id()
        );
        Ok(device_id)
    }
}
