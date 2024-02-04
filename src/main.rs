mod command_runner;
mod config;
mod device_id;
mod listener;
use anyhow::Result;

fn main() -> Result<()> {
    let config = config::Config::load()?;
    listener::start_listener(config)?;

    Ok(())
}
