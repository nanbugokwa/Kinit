#[cfg(feature = "modules-fstab")]
pub mod fstab;

#[cfg(feature = "modules-devices")]
pub mod devices;

#[cfg(feature = "modules-network")]
pub mod network;

#[cfg(feature = "recovery")]
pub mod recovery;

#[cfg(feature = "luks")]
pub mod crypto;

#[cfg(feature = "modules-desktop")]
pub mod desktop;
