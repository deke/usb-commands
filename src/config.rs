use anyhow::Result;
use dirs;
use ini;

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
        let config_file = Config::config_file_name().unwrap();
        print!("CONFIG FILE: {:?}", config_file);
        let config = ini::Ini::load_from_file(config_file).unwrap();
        let section = config.general_section();
        let device_id = section.get("device_id").unwrap();
        let execute_on_connect = section.get("execute_on_connect").unwrap();
        let execute_on_disconnect = section.get("execute_on_disconnect").unwrap();
        let config = Config::new(
            device_id.to_string(),
            execute_on_connect.to_string(),
            execute_on_disconnect.to_string(),
        );
        println!("{:?}", config);
        // Ok(Self::new(
        //     "2109:0103".to_string(),
        //     "echo connected".to_string(),
        //     "echo disconnected".to_string(),
        // ))
        Ok(config)
    }

    pub fn config_file_name() -> Result<std::path::PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| "Config directory not found")
            .unwrap()
            .join("dev.deke.usb-commands/usb-commands.ini");

        Ok(config_dir)
    }
}
