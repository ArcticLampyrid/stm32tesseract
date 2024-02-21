use std::{env, io, path::PathBuf};

pub fn get_resources_path() -> io::Result<PathBuf> {
    let mut path = env::current_exe()?;
    path.pop(); // File name
    if cfg!(debug_assertions) {
        path.pop(); // Debug folder
        path.pop(); // Target folder
        path.push("cli");
    } else {
        path.pop(); // Bin folder
    }
    path.push("resources");
    #[cfg(not(target_os = "windows"))]
    if !path.exists() {
        path = PathBuf::from("/var/lib/stm32tesseract/resources");
    }
    Ok(path)
}
