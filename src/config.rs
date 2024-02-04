use anyhow::{anyhow, Result};
use dirs;
use ini::{Ini, Properties};
use log::info;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub device_id: String,
    pub execute_on_connect: String,
    pub execute_on_disconnect: String,
}

impl Config {
    fn new(device_id: String, execute_on_connect: String, execute_on_disconnect: String) -> Self {
        Config {
            device_id,
            execute_on_connect,
            execute_on_disconnect,
        }
    }

    pub fn load() -> Result<Self> {
        let config_file = Config::config_file_name()?;
        info!("Loading config from: {:?}", config_file);

        let ini = Ini::load_from_file(config_file)?;
        let config = Config::build_config(ini)?;
        info!("Loaded config: {:?}", config);

        Ok(config)
    }

    fn build_config(ini: Ini) -> Result<Config> {
        let section = ini.general_section();
        let device_id = Config::extract_value(section, "device_id")?;
        let execute_on_connect = Config::extract_value(section, "execute_on_connect")?;
        let execute_on_disconnect = Config::extract_value(section, "execute_on_disconnect")?;
        let config = Config::new(device_id, execute_on_connect, execute_on_disconnect);

        Ok(config)
    }

    fn extract_value(section: &Properties, key: &str) -> Result<String> {
        section
            .get(key)
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("{} not found in config file", key))
    }

    pub fn config_file_name() -> Result<PathBuf> {
        let config_dir = dirs::config_dir().ok_or_else(|| anyhow!("Config directory not found"))?;

        Ok(config_dir.join("dev.deke.usb-commands/usb-commands.ini"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let input = r#"
        device_id = "0h1d:0a0a"
        execute_on_connect = "/usr/local/bin/m1ddc set input 15"
        execute_on_disconnect = "/usr/local/bin/m1ddc set input 17"
        "#;

        let ini = Ini::load_from_str(input).unwrap();
        let config = Config::build_config(ini).unwrap();

        assert_eq!(config.device_id, "0h1d:0a0a");
        assert_eq!(
            config.execute_on_connect,
            "/usr/local/bin/m1ddc set input 15"
        );
        assert_eq!(
            config.execute_on_disconnect,
            "/usr/local/bin/m1ddc set input 17"
        );
    }
}
