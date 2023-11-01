use std::fs;

use slint_build::CompilerConfiguration;

fn main() {
    let config = CompilerConfiguration::new().with_style("fluent-dark".to_string());
    slint_build::compile_with_config("ui/appwindow.slint", config).unwrap();

    // Embed the Windows resource file
    if let Ok(paths) = fs::read_dir("win32_resource") {
        for path in paths.flatten() {
            println!("cargo:rerun-if-changed={}", path.path().display());
        }
    }
    embed_resource::compile("win32_resource/app.rc", embed_resource::NONE);
}
