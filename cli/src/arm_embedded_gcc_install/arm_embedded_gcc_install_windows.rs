use crate::{
    download_manager::download_file, error::InstallError, gh_helper, path_env::add_to_path_env,
    reqwest_unified_builder,
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

    let client = reqwest_unified_builder::build_blocking()?;
    let url_remote = gh_helper::get_latest_release_url_with_fallback(
        &client,
        "xpack-dev-tools",
        "arm-none-eabi-gcc-xpack",
        |assert_name| assert_name.ends_with("-win32-x64.zip"),
        "https://github.com/xpack-dev-tools/arm-none-eabi-gcc-xpack/releases/download/v13.3.1-1.1/xpack-arm-none-eabi-gcc-13.3.1-1.1-win32-x64.zip",
    );
    let url_remote = gh_helper::elect_mirror(url_remote);

    println!("Downloading {}", url_remote);
    let path_local = download_file(&url_remote)?;
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
