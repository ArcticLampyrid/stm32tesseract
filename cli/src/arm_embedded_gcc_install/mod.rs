mod arm_embedded_gcc_install_windows;
use crate::error::InstallError;

pub fn install_arm_embedded_gcc() -> Result<(), InstallError> {
    if cfg!(target_os = "windows") {
        arm_embedded_gcc_install_windows::install_arm_embedded_gcc_windows()
    } else {
        Err(InstallError::OsNotSupported())
    }
}
