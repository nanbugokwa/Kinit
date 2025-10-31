use std::fs;
use std::thread;
use std::time::Duration;
use nix::mount::{mount, MsFlags};

pub fn start() {
    thread::spawn(|| {
        let mut seen = std::collections::HashSet::new();
        loop {
            if let Ok(dir) = fs::read_dir("/dev") {
                for entry in dir {
                    if let Ok(e) = entry {
                        let name = e.file_name();
                        if let Some(n) = name.to_str() {
                            if n.starts_with("sd") && n.len() > 3 && seen.insert(n.to_string()) {
                                let dev = format!("/dev/{}", n);
                                let mp = format!("/media/{}", n);
                                let _ = fs::create_dir_all(&mp);
                                let _ = mount(Some(&dev), &mp, Some("vfat"), MsFlags::empty(), None::<&str>);
                            }
                        }
                    }
                }
            }
            thread::sleep(Duration::from_secs(2));
        }
    });
}
