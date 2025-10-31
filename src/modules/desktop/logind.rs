use std::os::unix::net::UnixListener;
use std::thread;
use std::io::{Read, Write};

pub fn start() {
    thread::spawn(|| {
        let _ = std::fs::remove_file("/run/systemd/private");
        if let Ok(l) = UnixListener::bind("/run/systemd/private") {
            for stream in l.incoming() {
                if let Ok(mut s) = stream {
                    let mut buf = [0; 256];
                    if let Ok(n) = s.read(&mut buf) {
                        if String::from_utf8_lossy(&buf[..n]).contains("PowerOff") {
                            unsafe { libc::kill(1, libc::SIGTERM); }
                        }
                    }
                }
            }
        }
    });
}
