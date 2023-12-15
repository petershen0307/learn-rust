# Chapter 1

[A Freestanding Rust Binary](https://os.phil-opp.com/freestanding-rust-binary)

## How to build

[rust supported platform](https://doc.rust-lang.org/nightly/rustc/platform-support.html)

We can directly choose the target `x86_64-unknown-none` to build application without std library. We can also add a `config.toml` at `./.cargo` to specify build target to `x86_64-unknown-none`, so we can use `cargo build` to build application without std library.

* [config.toml](https://doc.rust-lang.org/cargo/reference/config.html)

```bash
rustup target add x86_64-unknown-none
cargo build --target x86_64-unknown-none
```

We can build without libc in link stage.

```bash
# Linux
cargo rustc -- -C link-arg=-nostartfiles
# Windows
cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
# macOS
cargo rustc -- -C link-args="-e __start -static -nostartfiles"
```
