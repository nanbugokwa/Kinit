use std::fs;
use nix::mount::{mount, MsFlags};

pub fn mount_all() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("/etc/fstab")?;
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            continue;
        }
        let device = parts[0];
        let mount_point = parts[1];
        let fs_type = parts[2];
        let options = parts[3];
        fs::create_dir_all(mount_point)?;
        let flags = parse_options(options);
        mount(Some(device), mount_point, Some(fs_type), flags, None::<&str>)?;
    }
    Ok(())
}

fn parse_options(opts: &str) -> MsFlags {
    let mut flags = MsFlags::empty();
    for opt in opts.split(',') {
        match opt {
            "ro" => flags |= MsFlags::MS_RDONLY,
            "noexec" => flags |= MsFlags::MS_NOEXEC,
            _ => {}
        }
    }
    flags
}
