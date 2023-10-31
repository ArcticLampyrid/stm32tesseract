use std::env;

use crate::{error::InstallError, gh_helper, path_env::add_to_path_env};
use zip::ZipArchive;

pub fn install_ninja_windows() -> Result<(), InstallError> {
    let client = reqwest::blocking::ClientBuilder::new()
        .user_agent("ninja-installer")
        .build()?;
    let url_for_ninja_win_zip = gh_helper::get_latest_release_url_with_fallback(
        &client,
        "ninja-build",
        "ninja",
        |assert_name| assert_name == "ninja-win.zip",
        "https://github.com/ninja-build/ninja/releases/download/v1.11.1/ninja-win.zip",
    );
    let url_for_ninja_win_zip = format!("https://ghproxy.com/{}", url_for_ninja_win_zip);

    println!("Downloading {}", url_for_ninja_win_zip);
    let response = client.get(url_for_ninja_win_zip).send()?;
    if !response.status().is_success() {
        return Err(InstallError::HttpStatusError(response.status()));
    }
    let content = response.bytes()?;

    println!("Extracting...");
    let mut zip = ZipArchive::new(std::io::Cursor::new(content))?;
    let mut ninja_exe_in_zip = zip.by_name("ninja.exe")?;
    let system_drive = env::var("SYSTEMDRIVE").unwrap_or("C:".to_string());
    let folder_path = format!("{}\\stm32tesseract_tools\\ninja", system_drive);
    std::fs::create_dir_all(&folder_path)?;
    let mut ninja_exe_local = std::fs::File::create(format!("{}\\ninja.exe", folder_path))?;
    std::io::copy(&mut ninja_exe_in_zip, &mut ninja_exe_local)?;

    println!("Setting up environment variables...");
    add_to_path_env(&folder_path)?;
    Ok(())
}
