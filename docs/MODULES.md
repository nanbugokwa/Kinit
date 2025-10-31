# How to Add a Module

1. Create a file in `src/modules/` or `src/platform/linux/`.
2. Wrap code in `#[cfg(feature = "my-feature")]`.
3. Add a hook in `core::init::run()` if needed.
4. Register the feature in `Cargo.toml`.
5. Add an example config in `examples/`.

Example:
```rust
// src/modules/hello.rs
#[cfg(feature = "hello")]
pub fn say_hello() {
    eprintln!("Hello from module!");
}


---

### `docs/SECURITY.md`
```markdown
# Security Model

- All file descriptors use `FD_CLOEXEC`
- Privileges dropped before `exec`
- Services run in isolated cgroups (if enabled)
- No shell invocation — commands parsed as argv
- TPM ensures boot integrity
- Recovery mode prevents permanent lockout
