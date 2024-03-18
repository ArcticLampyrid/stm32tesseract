use crate::{
    error::InstallError,
    pkg_manager::{install_via_package_manager, PackageItems, PackageManagerKind},
};

const PKG_ITEMS: &[PackageItems] = &[
    PackageItems::new(PackageManagerKind::Pacman, &["ninja"]),
    PackageItems::new(PackageManagerKind::AptGet, &["ninja-build"]),
    PackageItems::new(PackageManagerKind::Dnf, &["ninja-build"]),
    PackageItems::new(PackageManagerKind::Emerge, &["dev-util/ninja"]),
    PackageItems::new(PackageManagerKind::Zypper, &["ninja"]),
    PackageItems::new(PackageManagerKind::Apk, &["ninja"]),
    PackageItems::new(PackageManagerKind::XbpsInstall, &["ninja"]),
];

pub fn install_ninja_linux() -> Result<(), InstallError> {
    install_via_package_manager(PKG_ITEMS)
}
