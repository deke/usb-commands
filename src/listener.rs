use crate::command_runner;
use crate::config;
use crate::device_id::DeviceId;
use anyhow::Result;

use log::{debug, error, info};
use rusb::{Context, Device, UsbContext};

impl<T: UsbContext> rusb::Hotplug<T> for config::Config {
    fn device_arrived(&mut self, device: Device<T>) {
        handle_device_event(&device, &self.device_id, &self.execute_on_connect);
    }

    fn device_left(&mut self, device: Device<T>) {
        handle_device_event(&device, &self.device_id, &self.execute_on_disconnect);
    }
}

fn handle_device_event<T: UsbContext>(device: &Device<T>, device_id: &str, command_string: &str) {
    if device.device_id().unwrap_or_default() == device_id {
        debug!("device event: {:?}", device);
        let result = command_runner::run_command(command_string);

        match result {
            Ok(output) => info!("{}", output),
            Err(err) => error!("Error running command: '{}' {}", command_string, err),
        }
    }
}

pub fn start_listener(config: config::Config) -> Result<()> {
    if rusb::has_hotplug() {
        let context = Context::new()?;
        let _reg: Result<rusb::Registration<rusb::Context>, rusb::Error> =
            rusb::HotplugBuilder::new()
                .enumerate(true)
                .register(&context, Box::new(config));

        loop {
            match context.handle_events(None) {
                Ok(_) => {}
                Err(e) => {
                    error!("Error handling USB events: {:?}", e);
                }
            }
        }
    }

    Ok(())
}
