pub mod dbus;
pub mod logind;
pub mod udisks;
pub mod display;

pub fn start() {
    dbus::start();
    logind::start();
    udisks::start();
    display::start();
}
