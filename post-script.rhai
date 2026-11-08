// ch32v00x keeps both custom JSON specs until the concrete variant is known;
// drop the one that does not match the resolved target.
let target = variable::get("target");
if target == "riscv32ec-unknown-none-elf" {
    if file::exists("riscv32emc-unknown-none-elf.json") {
        file::delete("riscv32emc-unknown-none-elf.json");
    }
} else if target == "riscv32emc-unknown-none-elf" {
    if file::exists("riscv32ec-unknown-none-elf.json") {
        file::delete("riscv32ec-unknown-none-elf.json");
    }
} else {
    if file::exists("riscv32ec-unknown-none-elf.json") {
        file::delete("riscv32ec-unknown-none-elf.json");
    }
    if file::exists("riscv32emc-unknown-none-elf.json") {
        file::delete("riscv32emc-unknown-none-elf.json");
    }
}

let h417_dualcore = variable::get("h417_dualcore");
if h417_dualcore {
    // Replace the single-crate blink skeleton with the dual-core workspace.
    file::delete("src");
    file::delete("Cargo.toml");
    file::delete("build.rs");
    file::delete(".cargo");
    file::delete("memory-h417.x");
    file::delete("link-h417.x");
    file::delete("rust-toolchain.toml");
    file::delete("README-TEMPLATE.md");

    file::rename("h417-dual/Cargo.toml", "Cargo.toml");
    file::rename("h417-dual/rust-toolchain.toml", "rust-toolchain.toml");
    file::rename("h417-dual/README-TEMPLATE.md", "README.md");
    file::rename("h417-dual/.cargo", ".cargo");
    file::rename("h417-dual/v3f", "v3f");
    file::rename("h417-dual/v5f", "v5f");
    file::rename("h417-dual/xtask", "xtask");
    file::delete("h417-dual");
} else {
    if file::exists("h417-dual") {
        file::delete("h417-dual");
    }
    file::rename("README-TEMPLATE.md", "README.md");
}
