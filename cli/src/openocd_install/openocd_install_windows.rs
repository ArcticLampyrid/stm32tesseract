use crate::{error::InstallError, gh_helper, path_env::add_to_path_env};
use tar::Archive;

pub fn install_openocd_windows() -> Result<(), InstallError> {
    let client = reqwest::blocking::ClientBuilder::new()
        .user_agent("openocd-installer")
        .build()
        .map_err(InstallError::HttpFetchFailed)?;
    let url_for_openocd_win_tgz = gh_helper::get_latest_release_url_with_fallback(
        &client,
        "openocd-org",
        "openocd",
        |assert_name| assert_name.ends_with("-i686-w64-mingw32.tar.gz"),
        "https://github.com/openocd-org/openocd/releases/download/v0.12.0/openocd-v0.12.0-i686-w64-mingw32.tar.gz",
    );

    let url_for_openocd_win_tgz = format!("https://ghproxy.com/{}", url_for_openocd_win_tgz);
    println!("Downloading {}", url_for_openocd_win_tgz);
    let response = client.get(url_for_openocd_win_tgz).send()?;
    if !response.status().is_success() {
        return Err(InstallError::HttpStatusError(response.status()));
    }
    let content = response.bytes()?;

    println!("Extracting...");
    let mut archive = Archive::new(flate2::read::GzDecoder::new(std::io::Cursor::new(content)));
    let system_drive = std::env::var("SYSTEMDRIVE").unwrap_or("C:".to_string());
    let folder_path = format!("{}\\stm32tesseract_tools\\openocd", system_drive);
    std::fs::create_dir_all(&folder_path)?;
    archive.unpack(&folder_path)?;
    let openocd_bin_path = format!("{}\\bin", folder_path);

    println!("Setting up environment variables...");
    add_to_path_env(&openocd_bin_path)?;
    Ok(())
}
