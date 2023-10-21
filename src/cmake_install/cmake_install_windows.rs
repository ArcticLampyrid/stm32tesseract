use std::env;
use std::path::PathBuf;

use crate::path_env::add_to_path_env;
use crate::{error::InstallError, gh_helper};
use tempfile::tempdir;

use winreg::enums::*;
use winreg::RegKey;

fn find_install_folder_via_registry() -> Option<String> {
    let hkcu = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key_path = r"SOFTWARE\Kitware\CMake";
    let result_native = hkcu
        .open_subkey_with_flags(key_path, KEY_READ)
        .and_then(|x| x.get_value("InstallDir"))
        .ok();
    let result_wow64 = hkcu
        .open_subkey_with_flags(key_path, KEY_READ | KEY_WOW64_32KEY)
        .and_then(|x| x.get_value("InstallDir"))
        .ok();
    result_native.or(result_wow64)
}

pub fn install_cmake_windows() -> Result<(), InstallError> {
    if let Some(install_folder) = find_install_folder_via_registry() {
        println!("Found installed CMake via registry at {}", install_folder);
        println!("Patch: Setting up environment variables...");
        add_to_path_env(
            PathBuf::from(install_folder)
                .join("bin")
                .to_string_lossy()
                .as_ref(),
        )?;
        return Ok(());
    }

    let cmake_arch_suffix = match env::consts::ARCH {
        "x86" => "i386",
        "x86_64" => "x86_64",
        "aarch64" => "arm64",
        _ => return Err(InstallError::ArchNotSupported()),
    };
    let cmake_installer_suffix = format!("-windows-{}.msi", cmake_arch_suffix);

    let client = reqwest::blocking::ClientBuilder::new()
        .user_agent("cmake-installer")
        .build()?;
    let url_for_cmake_msi = gh_helper::get_latest_release_url_with_fallback(
        &client,
        "Kitware",
        "CMake",
        |assert_name| assert_name.ends_with(cmake_installer_suffix.as_str()),
        format!(
            "https://github.com/Kitware/CMake/releases/download/v3.27.7/cmake-3.27.7{}",
            cmake_installer_suffix
        )
        .as_str(),
    );
    let url_for_cmake_msi = format!("https://ghproxy.com/{}", url_for_cmake_msi);

    println!("Downloading {}", url_for_cmake_msi);
    let mut response = client.get(url_for_cmake_msi).send()?;
    if !response.status().is_success() {
        return Err(InstallError::HttpStatusError(response.status()));
    }

    let tempdir = tempdir()?;
    let path_local = {
        let mut path_local = tempdir.path().to_path_buf();
        path_local.push("cmake-installer.msi");
        path_local
    };
    {
        let mut downloaded_msi = std::fs::File::create(path_local.as_path())?;
        response.copy_to(&mut downloaded_msi)?;
    }

    println!("Installing...");
    let mut cmd = std::process::Command::new("msiexec");
    cmd.arg("/i")
        .arg(path_local.as_os_str())
        .arg("/qn")
        .arg("/norestart")
        .arg("ADD_CMAKE_TO_PATH=System")
        .arg("ALLUSERS=1");
    let status = cmd.status()?;
    if !status.success() {
        Err(InstallError::ExternalProgramFailed(status))
    } else {
        Ok(())
    }
}
