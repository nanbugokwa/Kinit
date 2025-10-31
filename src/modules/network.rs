use std::process::Command;

pub fn setup_network() -> Result<(), Box<dyn std::error::Error>> {
    Command::new("ip").args(["link", "set", "lo", "up"]).status()?;
    Command::new("ip").args(["addr", "add", "127.0.0.1/8", "dev", "lo"]).status()?;
    if Command::new("ip").args(["link", "show", "eth0"]).status().is_ok() {
        Command::new("ip").args(["link", "set", "eth0", "up"]).status()?;
        let _ = Command::new("udhcpc").arg("-i").arg("eth0").status();
    }
    Ok(())
}
