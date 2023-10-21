use std::io;

mod path_env_windows;
pub fn check_path_env_permission() -> io::Result<()> {
    if cfg!(target_os = "windows") {
        path_env_windows::check_path_env_permission_windows()
    } else {
        Ok(())
    }
}

pub fn add_to_path_env(new_item: &str) -> io::Result<()> {
    if cfg!(target_os = "windows") {
        path_env_windows::add_to_path_env_windows(new_item)
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Not implemented on this platform",
        ))
    }
}
