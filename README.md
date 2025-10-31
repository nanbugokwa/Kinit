# Kinit

A minimal, secure, and modular init system written in Rust.  
Supports servers, desktops, encrypted disks (LUKS), TPM, cgroups, recovery mode, and more.

WRITTEN BY Qwen AI, not tested

## Features
- PID 1 compliant (reaps zombies)
- Service dependencies & auto-restart
- Graceful shutdown (SIGTERM → SIGKILL)
- fstab mounting, device population
- LUKS decryption
- TPM PCR measurement
- Desktop support (D-Bus, logind stub, display managers)
- Recovery shell on boot failure

## Build
```bash
# Server-only
make build-server

# Full desktop
make build-desktop

# Create initramfs
make initramfs
