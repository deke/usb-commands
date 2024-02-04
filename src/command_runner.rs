use anyhow::Result;
use shellwords::split;
use std::process::Command;

pub fn run_command(command_string: &str) -> Result<String> {
    let mut args = split(command_string)?;
    let command = args.remove(0);
    let output = Command::new(command).args(args).output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_valid_command() {
        let command_string = "echo Hello, World!";
        let result = run_command(command_string);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, World!\n");
    }

    #[test]
    fn test_run_invalid_command() {
        let command_string = "nonexistent_command arg1 arg2";
        let result = run_command(command_string);
        assert!(result.is_err());
    }
}
