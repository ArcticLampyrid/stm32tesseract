use slint_build::CompilerConfiguration;

fn main() {
    let config = CompilerConfiguration::new().with_style("fluent-dark".to_string());
    slint_build::compile_with_config("ui/appwindow.slint", config).unwrap();
}
