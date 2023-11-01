#![cfg(target_os = "windows")]

use std::io;
use winreg::enums::*;
use winreg::RegKey;

pub fn check_path_env_permission() -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let env_key = hklm.open_subkey_with_flags(
        "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment",
        KEY_READ | KEY_WRITE,
    )?;
    env_key.set_value("Path", &env_key.get_value::<String, _>("Path")?)?;
    Ok(())
}

pub fn add_to_path_env(new_item: &str) -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let env_key = hklm.open_subkey_with_flags(
        "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment",
        KEY_READ | KEY_WRITE,
    )?;
    let current_path: String = env_key.get_value("Path").unwrap_or_default();
    if !current_path.contains(new_item) {
        let new_path = if current_path.ends_with(';') {
            format!("{}{}", current_path, new_item)
        } else {
            format!("{};{}", current_path, new_item)
        };
        env_key.set_value("Path", &new_path)?;
        Ok(())
    } else {
        Ok(())
    }
}
