use crate::{
    error::InstallError,
    pkg_manager::{install_via_package_manager, PackageItems, PackageManagerKind},
};

const PKG_ITEMS: &[PackageItems] = &[
    PackageItems::new(PackageManagerKind::Pacman, &["cmake"]),
    PackageItems::new(PackageManagerKind::AptGet, &["cmake"]),
];

pub fn install_cmake_linux() -> Result<(), InstallError> {
    install_via_package_manager(PKG_ITEMS)
}
