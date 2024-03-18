use which::which_global;

use crate::error::InstallError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PackageManagerKind {
    /// for Arch
    Pacman,
    /// for Debian/Ubuntu
    AptGet,
    /// for Fedora
    Dnf,
    /// for Gentoo
    Emerge,
    /// for OpenSUSE
    Zypper,
    /// for Alpine
    Apk,
    /// for Void
    XbpsInstall,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PackageItems<'a> {
    package_manager: PackageManagerKind,
    names: &'a [&'a str],
}

impl PackageItems<'_> {
    pub const fn new<'a>(
        package_manager: PackageManagerKind,
        names: &'a [&'a str],
    ) -> PackageItems<'a> {
        PackageItems {
            package_manager,
            names,
        }
    }
}

pub fn install_via_package_manager(alternatives: &[PackageItems]) -> Result<(), InstallError> {
    let mut cmd = std::process::Command::new("sudo");
    for item in alternatives {
        match item.package_manager {
            PackageManagerKind::Pacman => {
                if which_global("pacman").is_err() {
                    continue;
                }
                cmd.arg("pacman").arg("-S");
                for name in item.names {
                    cmd.arg(name);
                }
                cmd.arg("--noconfirm");
            }
            PackageManagerKind::AptGet => {
                if which_global("apt-get").is_err() {
                    continue;
                }
                cmd.arg("apt-get").arg("install");
                for name in item.names {
                    cmd.arg(name);
                }
                cmd.arg("-y");
            }
            PackageManagerKind::Dnf => {
                if which_global("dnf").is_err() {
                    continue;
                }
                cmd.arg("dnf").arg("install");
                for name in item.names {
                    cmd.arg(name);
                }
                cmd.arg("-y");
            }
            PackageManagerKind::Emerge => {
                if which_global("emerge").is_err() {
                    continue;
                }
                cmd.arg("emerge").arg("--ask=n");
                for name in item.names {
                    cmd.arg(name);
                }
            }
            PackageManagerKind::Zypper => {
                if which_global("zypper").is_err() {
                    continue;
                }
                cmd.arg("zypper").arg("--non-interactive").arg("install");
                for name in item.names {
                    cmd.arg(name);
                }
            }
            PackageManagerKind::Apk => {
                if which_global("apk").is_err() {
                    continue;
                }
                cmd.arg("apk").arg("add");
                for name in item.names {
                    cmd.arg(name);
                }
            }
            PackageManagerKind::XbpsInstall => {
                if which_global("xbps-install").is_err() {
                    continue;
                }
                cmd.arg("xbps-install").arg("-S");
                for name in item.names {
                    cmd.arg(name);
                }
                cmd.arg("--yes");
            }
        }
        let status = cmd.status()?;
        if status.success() {
            return Ok(());
        } else {
            return Err(InstallError::ExternalProgramFailed(status));
        }
    }
    Err(InstallError::SupportedPackageManagerNotFound())
}
