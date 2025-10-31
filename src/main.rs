#[cfg(feature = "tpm")]
mod tpm;

#[cfg(feature = "luks")]
mod crypto;

#[cfg(feature = "recovery")]
mod recovery;

mod core;
mod platform;
mod modules;
mod utils;

use std::env;

fn main() {
    // Ensure all FDs are CLOEXEC
    utils::fd::setup_cloexec();

    let config_path = env::args().nth(1).unwrap_or_else(|| "/etc/init.conf".into());
    let services = core::config::load(&config_path).expect("Failed to load config");

    core::init::run(services);
}
