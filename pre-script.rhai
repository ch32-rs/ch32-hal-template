let targets = #{
    ch32l103: "riscv32imc-unknown-none-elf",
    ch32v003: "riscv32ec-unknown-none-elf",
    ch32v103: "riscv32imc-unknown-none-elf",
    ch32v203: "riscv32imc-unknown-none-elf",
    ch32v208: "riscv32imc-unknown-none-elf",
    ch32v303: "riscv32imfc-unknown-none-elf",
    ch32v305: "riscv32imfc-unknown-none-elf",
    ch32v307: "riscv32imfc-unknown-none-elf",
    ch32x033: "riscv32imc-unknown-none-elf",
    ch32x035: "riscv32imc-unknown-none-elf",
    ch641: "riscv32ec-unknown-none-elf",
};

let embassy = variable::get("embassy");
let mcu_family = variable::get("mcu_family");
let target = targets.get(mcu_family);
variable::set("target", `${target}`);

// Handle families with only one variant
if mcu_family == "ch641" {
    variable::set("mcu", "ch641");
} else if mcu_family == "ch32x033" {
    variable::set("mcu", "ch32x033f8p6");
}

if embassy {
    variable::set("embassy_dep", true);
} else {
    // Even if embassy not selected, add embassy dependency for the V2/V3 because USB support depends on Embassy
    if mcu_family == "ch32v203" || mcu_family == "ch32v208" || mcu_family == "ch32v303" || mcu_family == "ch32v305" || mcu_family == "ch32v307" {
        variable::set("embassy_dep", true);
    } else {
        variable::set("embassy_dep", false);
    }
}
