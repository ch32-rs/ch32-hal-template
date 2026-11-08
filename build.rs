fn main() {
    println!("cargo:rustc-link-arg-bins=-Tlink.x");

    {% if mcu_family == "ch32h417" -%}
    // Override metapac's memory.x so qingke-rt sees a region named RAM.
    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::write(out_dir.join("memory.x"), include_bytes!("memory-h417.x")).unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=memory-h417.x");
    {%- endif %}
}
