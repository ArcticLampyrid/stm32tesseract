use crate::{
    download_manager::download_file, error::InstallError, path_env::add_to_path_env, pkgsrc,
};
use scopeguard::defer;
use tar::Archive;

pub fn install_openocd_windows() -> Result<(), InstallError> {
    let package = pkgsrc::fetch_package("openocd")?;
    let url_remote = package
        .match_asset(|assert| assert.name().ends_with("-i686-w64-mingw32.tar.gz"))?
        .download_url();
    println!("Downloading {}", url_remote);
    let path_local = download_file(url_remote)?;
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
