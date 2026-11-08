# {{ project-name }}

### Build

```bash
cargo build --release
```

### Flash

{% if use_probe_rs -%}
[probe-rs](https://probe.rs) needs to be installed:

```bash
cargo install probe-rs-tools --locked
```

With a debug probe connected to your target and then:

```bash
cargo run --release
```

The runner is `probe-rs run --chip {{ probe_rs_chip }}`. The `println!` in this
template writes to WCH's SDI console, which probe-rs does not decode; use a
defmt/RTT logger if you want logs over probe-rs.
{%- else -%}
[wlink](https://github.com/ch32-rs/wlink) needs to be installed:

```bash
cargo install --git https://github.com/ch32-rs/wlink
```

With a WCH-Link probe connected to your target and then:

```bash
cargo run --release
```
{%- endif %}

### Switching the flashing tool

The choice made at generation time only sets the default. Edit `runner` under
`[target."{{ target }}"]` in `.cargo/config.toml` to use the other tool:

```toml
# wlink
runner = "wlink -v flash --enable-sdi-print --watch-serial"

# probe-rs
runner = "probe-rs run --chip {{ probe_rs_chip }}{% if mcu_family == "ch32h417" %} --scan-region ram{% endif %}"
```

probe-rs addresses the part by name. Check `probe-rs chip list` if it reports
that `{{ probe_rs_chip }}` is unknown; a newer probe-rs may have added it,
otherwise keep using wlink.
