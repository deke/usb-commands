mod command_runner;
mod config;
mod device_id;
mod listener;
use anyhow::Result;
use log::info;

fn main() -> Result<()> {
    env_logger::init();
    info!("Starting usb-commands");
    let config = config::Config::load()?;
    listener::start_listener(config)?;

    Ok(())
}
