let output = `{
    "recommendations": [
        "ms-vscode.cpptools",
        "ms-vscode.cpptools-themes",
        "jeff-hykin.better-cpp-syntax",
        "ms-vscode.cmake-tools",
        "ms-vscode.vscode-serial-monitor",
        "marus25.cortex-debug",
`;
let lower_mcu = info.target_mcu.to_lower();
if lower_mcu.starts_with("stm32f1") {
    output += "        \"marus25.cortex-debug-dp-stm32f1\",\n";
} else if lower_mcu.starts_with("stm32f4") {
    output += "        \"marus25.cortex-debug-dp-stm32f4\",\n";
} else if lower_mcu.starts_with("stm32l4") {
    output += "        \"marus25.cortex-debug-dp-stm32l4\",\n";
} else if lower_mcu.starts_with("stm32h7") {
    output += "        \"marus25.cortex-debug-dp-stm32h7\",\n";
} else if lower_mcu.starts_with("stm32wl") {
    output += "        \"marus25.cortex-debug-dp-stm32wl\",\n";
}
output += `
        "dan-c-underwood.arm",
        "zixuanwang.linkerscript",
        "trond-snekvik.gnu-mapfiles"
    ]
}`;
output