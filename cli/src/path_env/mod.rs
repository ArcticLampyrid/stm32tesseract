#[cfg(target_os = "windows")]
mod path_env_windows;
#[cfg(target_os = "windows")]
pub use path_env_windows::*;

#[cfg(not(target_os = "windows"))]
pub fn check_path_env_permission() -> std::io::Result<()> {
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn add_to_path_env(_new_item: &str) -> std::io::Result<()> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Not implemented on this platform",
    ))
}

#[cfg(not(target_os = "windows"))]
pub fn get_path_env(_new_item: &str) -> Option<OsString> {
    std::env::var_os("PATH")
}
