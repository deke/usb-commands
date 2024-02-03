use crate::command_runner;
use crate::config;
use anyhow::Result;
use log::{debug, error};
use rusb::{Context, Device, UsbContext};

trait DeviceString {
    fn device_string(&self) -> Result<String>;
}

impl<T: UsbContext> DeviceString for Device<T> {
    fn device_string(&self) -> Result<String> {
        let device_desc = self.device_descriptor()?;
        let device_id = format!(
            "{:04x}:{:04x}",
            device_desc.vendor_id(),
            device_desc.product_id()
        );
        Ok(device_id)
    }
}

impl<T: UsbContext> rusb::Hotplug<T> for config::Config {
    fn device_arrived(&mut self, device: Device<T>) {
        handle_device_event(&device, &self.device_id, &self.execute_on_connect);
    }

    fn device_left(&mut self, device: Device<T>) {
        handle_device_event(&device, &self.device_id, &self.execute_on_disconnect);
    }
}

fn handle_device_event<T: UsbContext>(device: &Device<T>, device_id: &str, command: &str) {
    if device.device_string().unwrap() == device_id {
        debug!("device event: {:?}", device);
        command_runner::run_command(command);
    }
}
pub fn start_listener(config: config::Config) {
    if rusb::has_hotplug() {
        let context = Context::new().unwrap();
        let reg: Result<rusb::Registration<rusb::Context>, rusb::Error> =
            rusb::HotplugBuilder::new()
                .enumerate(true)
                .register(&context, Box::new(config));

        let _reg = Some(reg.unwrap());
        loop {
            match context.handle_events(None) {
                Ok(_) => {}
                Err(e) => {
                    error!("Error handling USB events: {:?}", e);
                    break;
                }
            }
        }
    }
}
