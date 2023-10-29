let output = "source [find interface/cmsis-dap.cfg]\n";
let lower_mcu = info.target_mcu.to_lower();
if lower_mcu.starts_with("stm32f0") {
    output += "source [find target/stm32f0x.cfg]\n";
} else if lower_mcu.starts_with("stm32f1") {
    output += "source [find target/stm32f1x.cfg]\n";
} else if lower_mcu.starts_with("stm32f2") {
    output += "source [find target/stm32f2x.cfg]\n";
} else if lower_mcu.starts_with("stm32f3") {
    output += "source [find target/stm32f3x.cfg]\n";
} else if lower_mcu.starts_with("stm32f4") {
    output += "source [find target/stm32f4x.cfg]\n";
} else if lower_mcu.starts_with("stm32f7") {
    output += "source [find target/stm32f7x.cfg]\n";
} else if lower_mcu.starts_with("stm32g0") {
    output += "source [find target/stm32g0x.cfg]\n";
} else if lower_mcu.starts_with("stm32g4") {
    output += "source [find target/stm32g4x.cfg]\n";
} else if lower_mcu.starts_with("stm32h7") {
    output += "source [find target/stm32h7x.cfg]\n";
} else if lower_mcu.starts_with("stm32l0") {
    output += "source [find target/stm32l0.cfg]\n";
} else if lower_mcu.starts_with("stm32l1") {
    output += "source [find target/stm32l1.cfg]\n";
} else if lower_mcu.starts_with("stm32l4") {
    output += "source [find target/stm32l4x.cfg]\n";
} else if lower_mcu.starts_with("stm32l5") {
    output += "source [find target/stm32l5x.cfg]\n";
} else if lower_mcu.starts_with("stm32u5") {
    output += "source [find target/stm32u5x.cfg]\n";
} else if lower_mcu.starts_with("stm32wb") {
    output += "source [find target/stm32wbx.cfg]\n";
} else if lower_mcu.starts_with("stm32wl") {
    output += "source [find target/stm32wlx.cfg]\n";
}
output