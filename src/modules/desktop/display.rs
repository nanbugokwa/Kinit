use std::process::Command;

pub fn start() {
    let dm = if std::fs::metadata("/usr/bin/gdm").is_ok() {
        "/usr/bin/gdm"
    } else if std::fs::metadata("/usr/bin/sddm").is_ok() {
        "/usr/bin/sddm"
    } else {
        return;
    };
    let _ = Command::new(dm).spawn();
}
