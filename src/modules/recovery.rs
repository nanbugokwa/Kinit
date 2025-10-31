use std::process::Command;
use nix::mount::{mount, MsFlags};

pub fn mount_root_or_recover() -> bool {
    if mount(
        Some("/dev/mapper/root"),
             "/",
             Some("ext4"),
             MsFlags::empty(),
             None::<&str>,
    ).is_ok() {
        return true;
    }
    launch_recovery();
    false
}

fn launch_recovery() {
    let _ = mount(Some("tmpfs"), "/", Some("tmpfs"), MsFlags::empty(), None::<&str>);
    for dir in ["bin", "dev", "proc", "sys"] {
        let _ = std::fs::create_dir_all(format!("/{}", dir));
    }
    let _ = mount(Some("proc"), "/proc", Some("proc"), MsFlags::empty(), None::<&str>);
    let _ = Command::new("/bin/sh").status();
}
