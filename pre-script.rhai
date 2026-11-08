let targets = #{
    ch32l103: "riscv32imc-unknown-none-elf",
    // ch32v00x target depends on the concrete variant (set below).
    ch32v103: "riscv32imc-unknown-none-elf",
    ch32v203: "riscv32imc-unknown-none-elf",
    ch32v208: "riscv32imc-unknown-none-elf",
    // V3xx has F but no A; use the closest standard target (imafc).
    ch32v303: "riscv32imafc-unknown-none-elf",
    ch32v305: "riscv32imafc-unknown-none-elf",
    ch32v307: "riscv32imafc-unknown-none-elf",
    ch32v407: "riscv32imc-unknown-none-elf",
    ch32v467: "riscv32imc-unknown-none-elf",
    ch32x033: "riscv32imc-unknown-none-elf",
    ch32x034: "riscv32imc-unknown-none-elf",
    ch32x035: "riscv32imc-unknown-none-elf",
    ch32h417: "riscv32imafc-unknown-none-elf",
    ch641: "riscv32ec-unknown-none-elf",
    ch643: "riscv32imc-unknown-none-elf",
};

let embassy = variable::get("embassy");
let mcu_family = variable::get("mcu_family");
let use_xwch_extensions = variable::get("use_xwch_extensions");

// Families with only one variant (or v3f-only for H417 in this template).
if mcu_family == "ch641" {
    variable::set("mcu", "ch641");
} else if mcu_family == "ch643" {
    variable::set("mcu", "ch643");
} else if mcu_family == "ch32x033" {
    variable::set("mcu", "ch32x033f8p6");
} else if mcu_family == "ch32h417" {
    variable::set("mcu", "ch32h417qeu6-v3f");
}

// Resolve the Rust target. ch32v00x covers both QingKe V2A (v003, RV32E)
// and V2C (v002/004/005/006/007/m007, RV32E+zmmul).
if mcu_family == "ch32v00x" {
    let mcu = variable::get("mcu");
    if mcu == "ch32v003a4m6" || mcu == "ch32v003f4p6"
        || mcu == "ch32v003f4u6" || mcu == "ch32v003j4m6" {
        variable::set("target", "riscv32ec-unknown-none-elf");
    } else {
        variable::set("target", "riscv32emc-unknown-none-elf");
    }
} else {
    let target = targets.get(mcu_family);
    variable::set("target", `${target}`);
}

// Optional WCH ISA extensions via rustflags on top of a standard rustup target.
// RV32E families (v00x/ch641) always use a custom JSON that already embeds
// +xwchc, so rustflags are unnecessary there.
let rv32e = mcu_family == "ch32v00x" || mcu_family == "ch641";
if use_xwch_extensions && !rv32e {
    variable::set("xwch_extensions", true);
    if mcu_family == "ch32v407" || mcu_family == "ch32v467" {
        variable::set("xwch_features", "+xwchc,+zve64x,+zvl64b,+zvbb");
    } else {
        variable::set("xwch_features", "+xwchc");
    }
} else {
    variable::set("xwch_extensions", false);
}

// CH32V4x7 rejects TIM time drivers (SysTick only). H417 examples also use SysTick.
if mcu_family == "ch32v407" || mcu_family == "ch32v467" || mcu_family == "ch32h417" {
    variable::set("time_driver_tim2", false);
} else {
    variable::set("time_driver_tim2", true);
}

if embassy {
    variable::set("embassy_dep", true);
} else {
    // Even if embassy is not selected, keep the dependency for families whose
    // USB support in ch32-hal currently depends on Embassy (USBFS uses
    // embassy_time::block_for).
    if mcu_family == "ch32l103"
        || mcu_family == "ch32v203" || mcu_family == "ch32v208"
        || mcu_family == "ch32v303" || mcu_family == "ch32v305" || mcu_family == "ch32v307"
        || mcu_family == "ch32v407" || mcu_family == "ch32v467"
        || mcu_family == "ch32h417" {
        variable::set("embassy_dep", true);
    } else {
        variable::set("embassy_dep", false);
    }
}
