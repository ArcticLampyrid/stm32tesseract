mod cmake_install_windows;
use crate::error::InstallError;

pub fn install_cmake() -> Result<(), InstallError> {
    if cfg!(target_os = "windows") {
        cmake_install_windows::install_cmake_windows()
    } else {
        Err(InstallError::OsNotSupported())
    }
}
