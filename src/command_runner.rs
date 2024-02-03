use anyhow::Result;
use shellwords::split;
use std::process::Command;

pub fn run_command(command_string: &str) {
    fn try_command(command: String, args: Vec<String>) -> Result<String> {
        let output = Command::new(command).args(args).output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
    let mut args = split(command_string).unwrap();
    let command = args.remove(0);
    let result = try_command(command, args);
    match result {
        Ok(output) => println!("{}", output),
        Err(err) => println!("Error running command: '{}' {}", command_string, err),
    }
}
