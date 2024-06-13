# {{ project-name }}

### Build

```bash
cargo build --release
```

### Flash

[wlink](https://github.com/ch32-rs/wlink) needs to be installed:

```bash
cargo install --git https://github.com/ch32-rs/wlink
```

With a WCH-Link probe connected to your target and then:

```bash
cargo run --release
```
