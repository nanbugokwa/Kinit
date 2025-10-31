use std::os::unix::net::UnixListener;
use std::thread;

pub fn start() {
    thread::spawn(|| {
        let _ = std::fs::remove_file("/run/dbus/system_bus_socket");
        if let Ok(l) = UnixListener::bind("/run/dbus/system_bus_socket") {
            for _ in l.incoming() {}
        }
    });
}
