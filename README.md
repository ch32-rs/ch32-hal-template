# Rust ch32-hal-template 

A template for use with [cargo-generate](https://github.com/cargo-generate/cargo-generate) to create applications targeting WCH's line of MCU.

## Generate the project

**Please make sure you have installed all [prerequisites](#prerequisites) first!**

To generate a project using this template:

```bash
cargo generate ch32-rs/ch32-hal-template
```

If `cargo generate` is not installed:

```bash
cargo install cargo-generate
```

## Build & Flash 

check [it](README-TEMPLATE.md)

If you need doc about you use [crates](https://crates.io/), run this command

```bash
cargo doc --open
```

## Prerequisites

> If you are new to rust embedded, you can refer to detailed instructions  [Setting Up a Development Environment](https://docs.espressif.com/projects/rust/book/getting-started/toolchain.html) chapter of The Rust on ESP Book.

### Install Rust (with `rustup`)

If you don't have `rustup` installed yet, follow the instructions on the [rustup.rs site](https://rustup.rs)

### Install Cargo Sub-Commands

```sh
cargo install cargo-generate
cargo install --git https://github.com/ch32-rs/wlink
```

> [!NOTE]
> [wlink](https://github.com/ch32-rs/wlink) needs to  be installed

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
