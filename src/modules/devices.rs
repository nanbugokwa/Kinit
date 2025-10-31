use std::fs;
use nix::sys::stat::Mode;
use nix::unistd::{mknod, MknodFlags};

pub fn populate_dev() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("/dev")?;
    create_dev("null", 1, 3, 0o666)?;
    create_dev("zero", 1, 5, 0o666)?;
    create_dev("random", 1, 8, 0o444)?;
    for i in 0..4 {
        create_dev(&format!("tty{}", i), 4, i, 0o600)?;
    }
    std::os::unix::fs::symlink("/proc/self/fd", "/dev/fd")?;
    Ok(())
}

fn create_dev(name: &str, major: u64, minor: u64, mode: u32) -> nix::Result<()> {
    let dev = nix::sys::stat::makedev(major, minor);
    mknod(&format!("/dev/{}", name), MknodFlags::S_IFCHR, Mode::from_bits(mode).unwrap(), dev)
}
