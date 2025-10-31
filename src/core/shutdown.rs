use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

pub struct ShutdownManager {
    pids: HashMap<String, Pid>,
}

impl ShutdownManager {
    pub fn new() -> Self {
        Self { pids: HashMap::new() }
    }

    pub fn register(&mut self, name: String, pid: Pid) {
        self.pids.insert(name, pid);
    }

    pub fn graceful_shutdown(self, timeout_sec: u64) {
        thread::spawn(move || {
            for (_, pid) in &self.pids {
                let _ = kill(*pid, Signal::SIGTERM);
            }
            thread::sleep(Duration::from_secs(timeout_sec));
            for (_, pid) in &self.pids {
                let _ = kill(*pid, Signal::SIGKILL);
            }
        });
    }
}
