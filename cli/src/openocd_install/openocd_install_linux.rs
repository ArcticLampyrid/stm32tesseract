use crate::{
    error::InstallError,
    pkg_manager::{install_via_package_manager, PackageItems, PackageManagerKind},
};

const PKG_ITEMS: &[PackageItems] = &[
    PackageItems::new(PackageManagerKind::Pacman, &["openocd"]),
    PackageItems::new(PackageManagerKind::AptGet, &["openocd"]),
];

pub fn install_openocd_linux() -> Result<(), InstallError> {
    install_via_package_manager(PKG_ITEMS)
}
