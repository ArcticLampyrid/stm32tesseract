mod ninja_install_linux;
mod ninja_install_macos;
mod ninja_install_windows;
use crate::error::InstallError;

pub fn install_ninja() -> Result<(), InstallError> {
    if cfg!(target_os = "linux") {
        ninja_install_linux::install_ninja_linux()
    } else if cfg!(target_os = "macos") {
        ninja_install_macos::install_ninja_macos()
    } else if cfg!(target_os = "windows") {
        ninja_install_windows::install_ninja_windows()
    } else {
        Err(InstallError::OsNotSupported())
    }
}
