mod arm_embedded_gcc_install_linux;
mod arm_embedded_gcc_install_windows;
use crate::error::InstallError;

pub fn install_arm_embedded_gcc() -> Result<(), InstallError> {
    if cfg!(target_os = "windows") {
        arm_embedded_gcc_install_windows::install_arm_embedded_gcc_windows()
    } else if cfg!(target_os = "linux") {
        arm_embedded_gcc_install_linux::install_arm_embedded_gcc_linux()
    } else {
        Err(InstallError::OsNotSupported())
    }
}
