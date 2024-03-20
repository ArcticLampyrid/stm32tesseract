mod openocd_install_linux;
mod openocd_install_windows;

use crate::error::InstallError;

pub fn install_openocd() -> Result<(), InstallError> {
    if cfg!(target_os = "windows") {
        openocd_install_windows::install_openocd_windows()
    } else if cfg!(target_os = "linux") {
        openocd_install_linux::install_openocd_linux()
    } else {
        Err(InstallError::OsNotSupported())
    }
}
