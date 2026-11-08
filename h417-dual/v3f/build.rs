fn main() {
    // Ship this crate's linker scripts. `memory.x` overrides the one metapac's
    // `memory-x` feature generates (which has no `RAM` region that qingke-rt's
    // link.x needs). `link.x` overrides qingke-rt's so the whole program runs
    // from the zero-wait shared SRAM.
    println!("cargo:rustc-link-arg-bins=-Tlink.x");

    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::write(out_dir.join("memory.x"), include_bytes!("memory.x")).unwrap();
    std::fs::write(out_dir.join("link.x"), include_bytes!("link.x")).unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=link.x");
}
