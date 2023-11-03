#![cfg(target_os = "windows")]

use once_cell::sync::Lazy;
use regex::Captures;
use regex::Regex;
use std::ffi::OsString;
use std::io;
use winreg::enums::*;
use winreg::RegKey;

static ENV_VAR: Lazy<Regex> = Lazy::new(|| Regex::new("%([[:word:]]+)%").expect("Invalid Regex"));

pub fn check_path_env_permission() -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let env_key = hklm.open_subkey_with_flags(
        "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment",
        KEY_READ | KEY_WRITE,
    )?;
    env_key.set_value("Path", &env_key.get_value::<String, _>("Path")?)?;
    Ok(())
}

fn get_user_latest_path_env() -> io::Result<OsString> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let env_key_system = hklm.open_subkey_with_flags(
        "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment",
        KEY_READ,
    )?;
    let env_key_user =
        RegKey::predef(HKEY_CURRENT_USER).open_subkey_with_flags("Environment", KEY_READ)?;
    let path_system: String = env_key_system.get_value("Path")?;
    let path_user: String = env_key_user.get_value("Path")?;
    let path_user_strip = path_user.strip_suffix(';').unwrap_or(&path_user);
    let path = if path_user_strip.is_empty() {
        path_system
    } else if path_system.ends_with(';') {
        format!("{}{}", path_system, path_user_strip)
    } else {
        format!("{};{}", path_system, path_user_strip)
    };
    let path_expanded: String = ENV_VAR
        .replace_all(path.as_str(), |captures: &Captures| {
            let var_name = &captures[1];
            env_key_user
                .get_value::<String, _>(var_name)
                .or_else(|_| env_key_system.get_value::<String, _>(var_name))
                .unwrap_or_else(|_| captures[0].to_string())
        })
        .into();
    Ok(path_expanded.into())
}

pub fn get_path_env() -> Option<OsString> {
    get_user_latest_path_env()
        .ok()
        .or_else(|| std::env::var_os("PATH"))
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
