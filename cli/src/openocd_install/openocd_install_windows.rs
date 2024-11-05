use crate::{
    download_manager::download_file, error::InstallError, gh_helper, path_env::add_to_path_env,
    reqwest_unified_builder,
};
use scopeguard::defer;
use tar::Archive;

pub fn install_openocd_windows() -> Result<(), InstallError> {
    let client = reqwest_unified_builder::build_blocking()?;
    let url_for_openocd_win_tgz = gh_helper::get_latest_release_url_with_fallback(
        &client,
        "openocd-org",
        "openocd",
        |assert_name| assert_name.ends_with("-i686-w64-mingw32.tar.gz"),
        "https://github.com/openocd-org/openocd/releases/download/v0.12.0/openocd-v0.12.0-i686-w64-mingw32.tar.gz",
    );

    let url_remote = gh_helper::elect_mirror(url_for_openocd_win_tgz);
    println!("Downloading {}", url_remote);
    let path_local = download_file(&url_remote)?;
    defer! {
        let _ = std::fs::remove_file(path_local.as_path());
    }

    println!("Extracting...");
    let mut archive = Archive::new(flate2::read::GzDecoder::new(std::fs::File::open(
        path_local.as_path(),
    )?));
    let system_drive = std::env::var("SYSTEMDRIVE").unwrap_or("C:".to_string());
    let folder_path = format!("{}\\stm32tesseract_tools\\openocd", system_drive);
    std::fs::create_dir_all(&folder_path)?;
    archive.unpack(&folder_path)?;

    println!("Setting up environment variables...");
    let bin_path = format!("{}\\bin", folder_path);
    add_to_path_env(&bin_path)?;
    Ok(())
}
