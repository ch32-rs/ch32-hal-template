fn main() {
    println!("cargo:rustc-link-arg-bins=-Tlink.x");

    {% if mcu_family == "ch32h417" -%}
    // Override metapac's memory.x so qingke-rt sees a region named RAM, and ship
    // the highcode link script so the V3F image runs from zero-wait shared SRAM
    // (hart 0 has no usable I-cache; Code FLASH is non-zero-wait).
    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::write(out_dir.join("memory.x"), include_bytes!("memory-h417.x")).unwrap();
    std::fs::write(out_dir.join("link.x"), include_bytes!("link-h417.x")).unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=memory-h417.x");
    println!("cargo:rerun-if-changed=link-h417.x");
    {%- endif %}
}
