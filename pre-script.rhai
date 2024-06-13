let targets = #{
    ch32l003: #{
        target: "riscv32imac-unknown-none-elf",
    },
    ch32v003: #{
        target: "riscv32ec-unknown-none-elf",
    },
    ch32v103: #{
        target: "riscv32imac-unknown-none-elf",
    },
    ch32v203: #{
        target: "riscv32imac-unknown-none-elf",
    },
    ch32v208: #{
        target: "riscv32imac-unknown-none-elf",
    },
    ch32v303: #{
        target: "riscv32imac-unknown-none-elf",
    },
    ch32v305: #{
        target: "riscv32imafc-unknown-none-elf",
    },
    ch32v307: #{
        target: "riscv32imafc-unknown-none-elf",
    },
    ch32x033: #{
        target: "riscv32imac-unknown-none-elf",
    },
    ch32x034: #{
        target: "riscv32imac-unknown-none-elf",
    },
    ch32x035: #{
        target: "riscv32imac-unknown-none-elf",
    },
};

let mcu_family = variable::get("mcu_family");
let target = targets.get(mcu_family).target;

variable::set("mcu_family", `${mcu_family}`);
variable::set("target", `${target}`);
