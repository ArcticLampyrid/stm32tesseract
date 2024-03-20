use crate::{
    error::InstallError,
    pkg_manager::{install_via_package_manager, PackageItems, PackageManagerKind},
};

const PKG_ITEMS: &[PackageItems] = &[
    PackageItems::new(
        PackageManagerKind::Pacman,
        &["arm-none-eabi-gcc", "arm-none-eabi-newlib"],
    ),
    PackageItems::new(
        PackageManagerKind::AptGet,
        &["gcc-arm-none-eabi", "libnewlib-arm-none-eabi"],
    ),
];

pub fn install_arm_embedded_gcc_linux() -> Result<(), InstallError> {
    install_via_package_manager(PKG_ITEMS)
}
