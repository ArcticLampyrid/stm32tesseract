mod cmake_install_linux;
mod cmake_install_windows;
use crate::error::InstallError;

pub fn install_cmake() -> Result<(), InstallError> {
    if cfg!(target_os = "windows") {
        cmake_install_windows::install_cmake_windows()
    } else if cfg!(target_os = "linux") {
        cmake_install_linux::install_cmake_linux()
    } else {
        Err(InstallError::OsNotSupported())
    }
}
