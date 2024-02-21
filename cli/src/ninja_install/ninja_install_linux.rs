use which::which_global;

use crate::error::InstallError;
pub fn install_ninja_linux() -> Result<(), InstallError> {
    /*
        Arch: pacman -S ninja --noconfirm
        Debian/Ubuntu: apt-get install ninja-build -y
        Fedora: dnf install ninja-build -y
        Gentoo: emerge --ask=n dev-util/ninja
        Opensuse: zypper --non-interactive install ninja
        Alpine: apk add ninja
        Void: xbps-install -S ninja --yes
    */
    let mut cmd = std::process::Command::new("sudo");
    if which_global("pacman").is_ok() {
        cmd.arg("pacman").arg("-S").arg("ninja").arg("--noconfirm");
        let status = cmd
            .status()
            .expect("failed to call pacman to install ninja");
        if !status.success() {
            Err(InstallError::ExternalProgramFailed(status))
        } else {
            Ok(())
        }
    } else if which_global("apt-get").is_ok() {
        cmd.arg("apt-get")
            .arg("install")
            .arg("ninja-build")
            .arg("-y");
        let status = cmd.status()?;
        if !status.success() {
            Err(InstallError::ExternalProgramFailed(status))
        } else {
            Ok(())
        }
    } else if which_global("dnf").is_ok() {
        cmd.arg("dnf").arg("install").arg("ninja-build").arg("-y");
        let status = cmd.status()?;
        if !status.success() {
            Err(InstallError::ExternalProgramFailed(status))
        } else {
            Ok(())
        }
    } else if which_global("emerge").is_ok() {
        cmd.arg("emerge").arg("--ask=n").arg("dev-util/ninja");
        let status = cmd.status()?;
        if !status.success() {
            Err(InstallError::ExternalProgramFailed(status))
        } else {
            Ok(())
        }
    } else if which_global("zypper").is_ok() {
        cmd.arg("zypper")
            .arg("--non-interactive")
            .arg("install")
            .arg("ninja");
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
        cmd.arg("xbps-install").arg("-S").arg("ninja").arg("--yes");
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
