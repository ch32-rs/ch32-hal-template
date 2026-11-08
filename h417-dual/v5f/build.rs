fn main() {
    // Ship this crate's memory map. qingke-rt's link.x wants `memory.x` (our V5F
    // FLASH/RAM/DTCM layout, overriding the one metapac generates). Do not ship
    // a `device.x` here — metapac's `rt` feature provides the interrupt vector
    // PROVIDEs, and this crate's link-search path would shadow it.
    println!("cargo:rustc-link-arg-bins=-Tlink.x");

    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::write(out_dir.join("memory.x"), include_bytes!("memory.x")).unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=memory.x");
}
