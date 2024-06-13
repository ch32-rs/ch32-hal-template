let targets = #{
    ch32l003: "riscv32imac-unknown-none-elf",
    ch32v003: "riscv32ec-unknown-none-elf",
    ch32v103: "riscv32imac-unknown-none-elf",
    ch32v203: "riscv32imac-unknown-none-elf",
    ch32v208: "riscv32imac-unknown-none-elf",
    ch32v303: "riscv32imac-unknown-none-elf",
    ch32v305: "riscv32imafc-unknown-none-elf",
    ch32v307: "riscv32imafc-unknown-none-elf",
    ch32x033: "riscv32imac-unknown-none-elf",
    ch32x034: "riscv32imac-unknown-none-elf",
    ch32x035: "riscv32imac-unknown-none-elf",
};

let mcu_family = variable::get("mcu_family");
let target = targets.get(mcu_family);

variable::set("target", `${target}`);
