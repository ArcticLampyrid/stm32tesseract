use std::{env, fs::File};

use crate::{
    download_manager::download_file, error::InstallError, path_env::add_to_path_env, pkgsrc,
};
use scopeguard::defer;
use zip::ZipArchive;

pub fn install_ninja_windows() -> Result<(), InstallError> {
    let platform_suffix = match env::consts::ARCH {
        "x86" => "win",
        "x86_64" => "win",
        "aarch64" => "winarm64",
        _ => return Err(InstallError::ArchNotSupported()),
    };
    let archive_name = format!("ninja-{}.zip", platform_suffix);
    let package = pkgsrc::fetch_package("ninja")?;
    let url_remote = package
        .match_asset(|assert| assert.name() == archive_name)?
        .download_url();

    println!("Downloading {}", url_remote);
    let path_local = download_file(url_remote)?;
    defer! {
        let _ = std::fs::remove_file(path_local.as_path());
    }

    println!("Extracting...");
    let mut zip = ZipArchive::new(File::open(path_local.as_path())?)?;
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
