use std::fs;
use std::process::Command;

pub fn unlock_root() {
    if let Ok(pass) = fs::read_to_string("/etc/luks.key") {
        let mut child = Command::new("cryptsetup")
        .args(["luksOpen", "--key-file", "-", "/dev/sda2", "root"])
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("cryptsetup failed");
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(pass.as_bytes()).ok();
        }
        child.wait().ok();
    }
}
