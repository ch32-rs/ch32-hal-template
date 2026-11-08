# {{ project-name }} (CH32H417 dual-core)

QingKe V3F (boot) + V5F workspace. Reset only starts the V3F; that image wakes
hart 1 at flash `0x00010000`.

### Build

```bash
cargo xtask build --example hello
```

### Flash

[wlink](https://github.com/ch32-rs/wlink) needs to be installed:

```bash
cargo install --git https://github.com/ch32-rs/wlink
```

```bash
cargo xtask flash --example hello
```

### Merge

Produces a combined `.bin` / `.hex` under `out/`:

```bash
cargo xtask merge --example hello
```

### Layout

| Core | Flash | RAM |
|------|-------|-----|
| V3F (boot) | `0x00000000` + 64K | shared SRAM `0x20100000` (highcode) |
| V5F | `0x00010000` + rest | DTCM `0x200C0200` |
