use which::which_global;

use crate::error::InstallError;
pub fn install_ninja_linux() -> Result<(), InstallError> {
    /*
        Arch: pacman -S ninja
        Debian/Ubuntu: apt-get install ninja-build
        Fedora: dnf install ninja-build
        Gentoo: emerge dev-util/ninja
        Opensuse: zypper in ninja
        Alpine: apk add ninja
        Void: xbps-install -S ninja
    */
    let mut cmd = std::process::Command::new("sudo");
    if which_global("pacman").is_ok() {
        cmd.arg("pacman").arg("-S").arg("ninja");
        let status = cmd
            .status()
            .expect("failed to call pacman to install ninja");
        if !status.success() {
            Err(InstallError::ExternalProgramFailed(status))
        } else {
            Ok(())
        }
    } else if which_global("apt-get").is_ok() {
        cmd.arg("apt-get").arg("install").arg("ninja-build");
        let status = cmd.status()?;
        if !status.success() {
            Err(InstallError::ExternalProgramFailed(status))
        } else {
            Ok(())
        }
    } else if which_global("dnf").is_ok() {
        cmd.arg("dnf").arg("install").arg("ninja-build");
        let status = cmd.status()?;
        if !status.success() {
            Err(InstallError::ExternalProgramFailed(status))
        } else {
            Ok(())
        }
    } else if which_global("emerge").is_ok() {
        cmd.arg("emerge").arg("dev-util/ninja");
        let status = cmd.status()?;
        if !status.success() {
            Err(InstallError::ExternalProgramFailed(status))
        } else {
            Ok(())
        }
    } else if which_global("zypper").is_ok() {
        cmd.arg("zypper").arg("in").arg("ninja");
        let status = cmd.status()?;
        if !status.success() {
            Err(InstallError::ExternalProgramFailed(status))
        } else {
            Ok(())
        }
    } else if which_global("apk").is_ok() {
        cmd.arg("apk").arg("add").arg("ninja");
        let status = cmd.status()?;
        if !status.success() {
            Err(InstallError::ExternalProgramFailed(status))
        } else {
            Ok(())
        }
    } else if which_global("xbps-install").is_ok() {
        cmd.arg("xbps-install").arg("-S").arg("ninja");
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
