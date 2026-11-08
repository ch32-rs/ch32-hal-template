# {{ project-name }} (CH32H417 dual-core)

QingKe V3F (boot) + V5F workspace. Reset only starts the V3F; that image wakes
hart 1 at flash `0x00010000`.

### Build

```bash
cargo xtask build --example hello
```

### Flash

{% if use_probe_rs -%}
[probe-rs](https://probe.rs) is the default flasher here:

```bash
cargo install probe-rs-tools --locked
```
{%- else -%}
[wlink](https://github.com/ch32-rs/wlink) is the default flasher here:

```bash
cargo install --git https://github.com/ch32-rs/wlink
```
{%- endif %}

```bash
cargo xtask flash --example hello
```

Both tools are supported; `--flasher` overrides the generated default per run:

```bash
cargo xtask flash --example hello --flasher wlink     # WCH's tool, resets on the last image
cargo xtask flash --example hello --flasher probe-rs  # verifies every image, resets once
```

`wlink` picks the first attached probe by itself; set `WLINK_PROBE` (or
`PROBE_RS_PROBE`, which both read) when more than one probe is connected.
`probe-rs` cannot choose between probes on its own, so it needs
`PROBE_RS_PROBE` in that case.

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
