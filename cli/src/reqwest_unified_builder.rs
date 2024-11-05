pub fn build_blocking() -> Result<reqwest::blocking::Client, reqwest::Error> {
    reqwest::blocking::ClientBuilder::new()
        .user_agent(concat!(
            "Mozilla/5.0 (Generic) STM32Tesseract/",
            env!("CARGO_PKG_VERSION")
        ))
        .build()
}
