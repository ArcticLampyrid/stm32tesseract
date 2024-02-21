pub fn build_blocking() -> Result<reqwest::blocking::Client, reqwest::Error> {
    reqwest::blocking::ClientBuilder::new()
        .user_agent("stm32tesseract")
        .build()
}
