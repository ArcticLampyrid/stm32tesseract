static BLOCKING_CLIENT: std::sync::OnceLock<reqwest::blocking::Client> = std::sync::OnceLock::new();

fn build_blocking_client() -> reqwest::blocking::Client {
    reqwest::blocking::ClientBuilder::new()
        .user_agent(concat!(
            "Mozilla/5.0 (Generic) STM32Tesseract/",
            env!("CARGO_PKG_VERSION")
        ))
        .build()
        .unwrap()
}

pub fn blocking_client() -> &'static reqwest::blocking::Client {
    BLOCKING_CLIENT.get_or_init(build_blocking_client)
}
