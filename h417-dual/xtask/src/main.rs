//! Build, flash and merge driver for the CH32H417 dual-core template.
//!
//! ```text
//! cargo xtask build [--example NAME]
//! cargo xtask flash --example NAME [--no-build]
//! cargo xtask merge --example NAME [--out DIR]
//! ```
//!
//! Both cores are always built/flashed together for the named example. Reset only
//! starts the V3F; the V5F half is woken by that image.

use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::{env, fs};

/// Bytes an erased flash byte reads as, used to pad the merged image.
const ERASED: u8 = 0xFF;

struct Core {
    name: &'static str,
    target: &'static str,
    flash_base: u64,
    flash_limit: u64,
}

const CORES: [Core; 2] = [
    Core {
        name: "v3f",
        target: "riscv32imafc-unknown-none-elf",
        flash_base: 0x0000_0000,
        flash_limit: 64 * 1024,
    },
    Core {
        name: "v5f",
        target: "riscv32imafbc-unknown-none-elf",
        flash_base: 0x0001_0000,
        flash_limit: 960 * 1024 - 64 * 1024,
    },
];

const USAGE: &str = "\
usage: cargo xtask <command> [options]

commands:
  build [--example NAME]                 build both cores (default example: hello)
  flash --example NAME [--no-build]      build, then flash V3F then V5F
  merge --example NAME [--out DIR]       merge the images into .bin + .hex

WLINK_PROBE=VID:PID:SERIAL (or a wlink index) pins every wlink call to one probe.
";

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let Some((command, rest)) = args.split_first() else {
        fail(USAGE);
    };

    let opts = Options::parse(rest);
    match command.as_str() {
        "build" => build(&opts),
        "flash" => flash(&opts),
        "merge" => merge(&opts),
        "help" | "--help" | "-h" => println!("{USAGE}"),
        other => fail(&format!("unknown command `{other}`\n\n{USAGE}")),
    }
}

struct Options {
    example: String,
    no_build: bool,
    out: Option<PathBuf>,
}

impl Options {
    fn parse(args: &[String]) -> Options {
        let mut opts = Options {
            example: "hello".into(),
            no_build: false,
            out: None,
        };
        let mut args = args.iter();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--example" => {
                    opts.example = args
                        .next()
                        .cloned()
                        .unwrap_or_else(|| fail("`--example` needs a name"));
                }
                "--out" => opts.out = args.next().map(PathBuf::from),
                "--no-build" => opts.no_build = true,
                other => fail(&format!("unknown option `{other}`\n\n{USAGE}")),
            }
        }
        opts
    }
}

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask lives one level below the dual-core project")
        .to_path_buf()
}

fn example_source(core: &Core, example: &str) -> PathBuf {
    root()
        .join(core.name)
        .join("src/bin")
        .join(format!("{example}.rs"))
}

fn example_elf(core: &Core, example: &str) -> PathBuf {
    // Workspace members share the root `target/` directory.
    root()
        .join("target")
        .join(core.target)
        .join("release")
        .join(example)
}

fn require_source(core: &Core, example: &str) {
    if !example_source(core, example).exists() {
        fail(&format!(
            "`{example}` has no {}-core half ({})",
            core.name,
            example_source(core, example).display()
        ));
    }
}

fn wlink_selector_args() -> Vec<String> {
    match env::var("WLINK_PROBE").or_else(|_| env::var("PROBE_RS_PROBE")) {
        Ok(selector) if !selector.is_empty() => vec!["-d".to_string(), selector],
        _ => Vec::new(),
    }
}

fn run(program: &str, args: &[&str], dir: &Path) {
    println!("  {} {}", program, args.join(" "));
    let status = Command::new(program)
        .args(args)
        .current_dir(dir)
        .status()
        .unwrap_or_else(|e| fail(&format!("failed to run `{program}`: {e}")));
    if !status.success() {
        fail(&format!("`{program}` failed with {status}"));
    }
}

fn run_wlink(args: &[&str], dir: &Path) {
    let selector = wlink_selector_args();
    let mut all: Vec<&str> = selector.iter().map(String::as_str).collect();
    all.extend_from_slice(args);
    run("wlink", &all, dir);
}

fn build(opts: &Options) {
    for core in &CORES {
        require_source(core, &opts.example);
        let dir = root().join(core.name);
        println!("building {} ({}) ...", core.name, opts.example);
        run(
            "cargo",
            &["build", "--release", "--bin", &opts.example],
            &dir,
        );
    }
    println!("built: v3f:{}+v5f:{}", opts.example, opts.example);
}

fn flash(opts: &Options) {
    for core in &CORES {
        require_source(core, &opts.example);
    }
    if !opts.no_build {
        build(opts);
    }
    // V3F first with --no-run so the chip stays halted; the final V5F write
    // resets and runs, by which time both images are in place.
    for (i, core) in CORES.iter().enumerate() {
        let elf = example_elf(core, &opts.example);
        println!("flashing {} ({}) ...", core.name, opts.example);
        let path = elf.to_str().expect("utf-8 path");
        if i + 1 == CORES.len() {
            run_wlink(&["flash", path], &root());
        } else {
            run_wlink(&["flash", "--no-run", path], &root());
        }
    }
}

struct Segment {
    core: &'static str,
    base: u64,
    bytes: Vec<u8>,
}

fn merge(opts: &Options) {
    for core in &CORES {
        require_source(core, &opts.example);
    }

    let segments: Vec<Segment> = CORES
        .iter()
        .map(|core| Segment {
            core: core.name,
            base: core.flash_base,
            bytes: flatten(&example_elf(core, &opts.example), core),
        })
        .collect();

    let end = segments
        .iter()
        .map(|s| s.base + s.bytes.len() as u64)
        .max()
        .expect("at least one segment");
    let mut image = vec![ERASED; end as usize];
    for segment in &segments {
        let at = segment.base as usize;
        image[at..at + segment.bytes.len()].copy_from_slice(&segment.bytes);
    }

    let out_dir = opts.out.clone().unwrap_or_else(|| root().join("out"));
    fs::create_dir_all(&out_dir)
        .unwrap_or_else(|e| fail(&format!("cannot create {}: {e}", out_dir.display())));
    let bin = out_dir.join(format!("{}.bin", opts.example));
    let hex = out_dir.join(format!("{}.hex", opts.example));
    fs::write(&bin, &image)
        .unwrap_or_else(|e| fail(&format!("cannot write {}: {e}", bin.display())));
    fs::write(&hex, intel_hex(&segments))
        .unwrap_or_else(|e| fail(&format!("cannot write {}: {e}", hex.display())));

    for segment in &segments {
        println!(
            "  {:<4} {:#010x}  {:>7} bytes",
            segment.core,
            segment.base,
            segment.bytes.len()
        );
    }
    println!("merged image: {} ({} bytes)", bin.display(), image.len());
    println!("intel hex:    {}", hex.display());
}

fn flatten(elf: &Path, core: &Core) -> Vec<u8> {
    if !elf.exists() {
        fail(&format!(
            "{} is not built — run `cargo xtask build --example {}`",
            elf.display(),
            elf.file_stem().unwrap_or_default().to_string_lossy()
        ));
    }
    let tmp = env::temp_dir().join(format!("h417-{}-{}.bin", core.name, std::process::id()));
    let objcopy = objcopy();
    run(
        &objcopy,
        &[
            "-O",
            "binary",
            "--gap-fill",
            "0xFF",
            elf.to_str().unwrap(),
            tmp.to_str().unwrap(),
        ],
        &root(),
    );
    let bytes =
        fs::read(&tmp).unwrap_or_else(|e| fail(&format!("cannot read {}: {e}", tmp.display())));
    let _ = fs::remove_file(&tmp);

    if bytes.len() as u64 > core.flash_limit {
        fail(&format!(
            "{}: {} bytes exceeds its {} byte flash region — it would overlap the other image",
            core.name,
            bytes.len(),
            core.flash_limit
        ));
    }
    bytes
}

fn objcopy() -> String {
    tool("llvm-objcopy")
}

fn tool(name: &str) -> String {
    let output = |args: &[&str]| {
        Command::new("rustc")
            .args(args)
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
    };
    if let (Some(sysroot), Some(host)) = (output(&["--print", "sysroot"]), output(&["-vV"])) {
        let host = host
            .lines()
            .find_map(|line| line.strip_prefix("host: ").map(str::to_string));
        if let Some(host) = host {
            let path = Path::new(&sysroot)
                .join("lib/rustlib")
                .join(host)
                .join("bin")
                .join(name);
            if path.exists() {
                return path.to_string_lossy().into_owned();
            }
        }
    }
    name.to_string()
}

fn intel_hex(segments: &[Segment]) -> String {
    let mut hex = String::new();
    let mut upper = u32::MAX;
    for segment in segments {
        for (i, chunk) in segment.bytes.chunks(16).enumerate() {
            let address = segment.base as u32 + (i * 16) as u32;
            if address >> 16 != upper {
                upper = address >> 16;
                hex.push_str(&record(0x04, 0, &[(upper >> 8) as u8, upper as u8]));
            }
            hex.push_str(&record(0x00, (address & 0xFFFF) as u16, chunk));
        }
    }
    hex.push_str(":00000001FF\n");
    hex
}

fn record(record_type: u8, address: u16, data: &[u8]) -> String {
    let mut bytes = vec![data.len() as u8, (address >> 8) as u8, address as u8, record_type];
    bytes.extend_from_slice(data);
    let sum = bytes.iter().fold(0u8, |acc, b| acc.wrapping_add(*b));
    let checksum = (!sum).wrapping_add(1);

    let mut line = format!(":{bytes:02X?}");
    line.retain(|c| !matches!(c, '[' | ']' | ' ' | ','));
    format!("{line}{checksum:02X}\n")
}

fn fail(message: &str) -> ! {
    eprintln!("xtask: {message}");
    exit(1);
}
