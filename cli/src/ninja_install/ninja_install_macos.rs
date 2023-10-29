use which::which_global;

use crate::error::InstallError;
pub fn install_ninja_macos() -> Result<(), InstallError> {
    if which_global("brew").is_ok() {
        let mut cmd = std::process::Command::new("brew");
        cmd.arg("install").arg("ninja");
        let status = cmd.status()?;
        if !status.success() {
            Err(InstallError::ExternalProgramFailed(status))
        } else {
            Ok(())
        }
    } else if which_global("port").is_ok() {
        let mut cmd = std::process::Command::new("port");
        cmd.arg("install").arg("ninja");
        let status = cmd.status()?;
        if !status.success() {
            Err(InstallError::ExternalProgramFailed(status))
        } else {
            Ok(())
        }
    } else {
        Err(InstallError::SupportedPackageManagerNotFound())
    }
}
