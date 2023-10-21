mod arm_embedded_gcc_install;
mod cmake_install;
mod cproject_reader;
mod error;
mod gh_helper;
mod ninja_install;
mod openocd_install;
mod path_env;

use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use error::InstallError;
use which::which_global;

use crate::{
    arm_embedded_gcc_install::install_arm_embedded_gcc, cmake_install::install_cmake,
    cproject_reader::read_cproject_file, ninja_install::install_ninja,
    openocd_install::install_openocd, path_env::check_path_env_permission,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Up {},
    Tesseract {
        #[clap(short, long)]
        file: String,
    },
}

fn check_and_install<I>(name: &str, command: &str, install: I) -> Result<(), InstallError>
where
    I: FnOnce() -> Result<(), InstallError>,
{
    println!("====== {} ======", name);
    match which_global(command) {
        Ok(path) => {
            println!("Found: {}", path.display());
            Ok(())
        }
        Err(_) => {
            println!("Not found, installing {}...", name);

            if let Err(e) = check_path_env_permission() {
                if cfg!(not(debug_assertions)) {
                    println!("Permission denied to modify environment variables");
                    return Err(InstallError::IOFailed(e));
                }
                println!("Permission denied to modify environment variables, continue anyway in debug mode");
            }

            if let Err(e) = install() {
                println!("Error {}", e);
                Err(e)
            } else {
                Ok(())
            }
        }
    }
}

fn command_set_up() {
    let mut okey = true;
    okey &= check_and_install("Ninja", "ninja", install_ninja).is_ok();
    okey &= check_and_install("OpenOCD", "openocd", install_openocd).is_ok();
    okey &= check_and_install(
        "GNU Arm Embedded GCC",
        "arm-none-eabi-gcc",
        install_arm_embedded_gcc,
    )
    .is_ok();
    okey &= check_and_install("CMake", "cmake", install_cmake).is_ok();
    println!("====== Conclusion ======");
    if okey {
        println!("All good");
        println!("Note: You may need to reboot your computer to apply new environment variables")
    } else {
        println!("Something went wrong");
        std::process::exit(1);
    }
}

fn command_tesseract(file: &str) {
    let mut cproject_path = PathBuf::from(file);
    if !cproject_path.is_dir() {
        cproject_path.pop();
    }
    cproject_path.push(".cproject");
    let cproject_path = fs::canonicalize(cproject_path).expect("Failed to resolve .cproject");
    let info = read_cproject_file(cproject_path.as_path()).expect("Failed to read cproject file");
    println!("====== Info ======");
    println!("CProjectFile: {}", cproject_path.display());
    println!("Project name: {}", info.project_name);
    println!("Target MCU: {}", info.target_mcu);
    println!(
        "Target CPU: {}",
        info.target_cpu.unwrap_or_else(|| "Unknown".to_string())
    );
    println!("Linker script: {}", info.linker_script);
    println!("Defined symbols: {:?}", info.defined_symbols);
    println!("Include paths: {:?}", info.include_paths);
    println!("Source entries: {:?}", info.source_entries);
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Up {} => {
            command_set_up();
        }
        Commands::Tesseract { file } => {
            command_tesseract(file.as_str());
        }
    }
}
