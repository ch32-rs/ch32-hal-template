file::rename("README-TEMPLATE.md", "README.md");

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
