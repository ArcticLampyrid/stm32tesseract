mod openocd_install_windows;

use crate::error::InstallError;

pub fn install_openocd() -> Result<(), InstallError> {
    if cfg!(target_os = "windows") {
        openocd_install_windows::install_openocd_windows()
    } else {
        Err(InstallError::OsNotSupported())
    }
}
