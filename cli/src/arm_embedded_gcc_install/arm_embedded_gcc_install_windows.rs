use crate::{
    download_manager::download_file, error::InstallError, path_env::add_to_path_env, pkgsrc,
};
use scopeguard::defer;
use std::{collections::HashSet, env, fs::File, path::PathBuf};
use zip::ZipArchive;

fn get_top_folders<R>(archive_reader: &mut ZipArchive<R>) -> Result<HashSet<String>, InstallError>
where
    R: std::io::Seek,
    R: std::io::Read,
{
    let mut top_folders = HashSet::new();

    for i in 0..archive_reader.len() {
        let entry = archive_reader
            .by_index(i)
            .map_err(|_| InstallError::MetadataError())?;
        if entry.name().contains('/') {
            let top_folder = entry.name().split('/').next().unwrap().to_string();
            top_folders.insert(top_folder);
        }
    }

    Ok(top_folders)
}

fn find_install_folder_via_registry() -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::*;
        use winreg::RegKey;
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let software = hklm
            .open_subkey_with_flags("SOFTWARE\\ARM", KEY_READ | KEY_WOW64_32KEY)
            .ok()?;

        software
            .enum_keys()
            .filter_map(|result| result.ok())
            .find_map(|key| {
                if key.contains("arm-none-eabi") {
                    let subkey_path = format!("SOFTWARE\\ARM\\{}", key);
                    let subkey = hklm
                        .open_subkey_with_flags(subkey_path, KEY_READ | KEY_WOW64_32KEY)
                        .ok()?;
                    subkey.get_value("InstallFolder").ok()
                } else {
                    None
                }
            })
    }
    #[cfg(not(target_os = "windows"))]
    {
        None
    }
}

pub fn install_arm_embedded_gcc_windows() -> Result<(), InstallError> {
    if let Some(install_folder) = find_install_folder_via_registry() {
        println!(
            "Found installed GNU Arm Embedded GCC via registry at {}",
            install_folder
        );
        println!("Patch: Setting up environment variables...");
        add_to_path_env(
            PathBuf::from(install_folder)
                .join("bin")
                .to_string_lossy()
                .as_ref(),
        )?;
        return Ok(());
    }

    if env::consts::ARCH == "x86" {
        return Err(InstallError::ArchNotSupported());
    }
    let package = pkgsrc::fetch_package("arm-none-eabi-gcc-xpack")?;
    let url_remote = package
        .match_asset(|assert| assert.name().ends_with("-win32-x64.zip"))?
        .download_url();

    println!("Downloading {}", url_remote);
    let path_local = download_file(url_remote)?;
    defer! {
        let _ = std::fs::remove_file(path_local.as_path());
    }

    println!("Extracting...");
    let mut zip = ZipArchive::new(File::open(path_local.as_path())?)?;
    let top_folders = get_top_folders(&mut zip)?;
    if top_folders.len() != 1 {
        return Err(InstallError::MetadataError());
    }
    let toolchain_name = top_folders.iter().next().unwrap();
    println!("Toolchain name: {}", toolchain_name);

    let system_drive = env::var("SYSTEMDRIVE").unwrap_or("C:".to_string());
    zip.extract(format!("{}\\stm32tesseract_tools", system_drive))
        .map_err(InstallError::InvalidZipArchive)?;

    println!("Setting up environment variables...");
    let bin_path = format!(
        "{}\\stm32tesseract_tools\\{}\\bin",
        system_drive, toolchain_name
    );
    add_to_path_env(&bin_path)?;
    Ok(())
}
