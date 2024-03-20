#![windows_subsystem = "windows"]
mod command_params;
use clap::Parser;
use command_params::CommandParams;
use once_cell::sync::Lazy;
use portable_pty::{native_pty_system, CommandBuilder, ExitStatus, PtySize};
use rfd::FileDialog;
use slint::Weak;
use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
};

slint::include_modules!();

const CARGO_PKG_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

static PATH_OF_CLI: Lazy<PathBuf> = Lazy::new(|| {
    let mut path_of_cli = std::env::current_exe().expect("Failed to get current exe path");
    path_of_cli.pop();
    if cfg!(target_os = "windows") {
        path_of_cli.push("stm32tesseract.exe");
    } else {
        path_of_cli.push("stm32tesseract");
    }
    if path_of_cli.exists() {
        return path_of_cli;
    }

    if cfg!(target_os = "windows") {
        PathBuf::from("stm32tesseract.exe")
    } else {
        PathBuf::from("stm32tesseract")
    }
});

fn do_env_check(ui_handle: &Weak<AppWindow>) {
    let mut cmd = CommandBuilder::new(PATH_OF_CLI.as_os_str());
    cmd.arg("env");
    cmd.arg("check");
    let ui_handle_2 = ui_handle.clone();
    execute_cmd(ui_handle, cmd, true, move |status| {
        if status.success() {
            let _ = ui_handle_2.upgrade_in_event_loop(move |ui| {
                ui.set_env_status("OK".into());
            });
        } else {
            let _ = ui_handle_2.upgrade_in_event_loop(move |ui| {
                ui.set_env_status("Failed".into());
            });
        }
    });
}

fn do_env_up(ui_handle: &Weak<AppWindow>) {
    #[cfg(target_os = "windows")]
    if cfg!(not(debug_assertions)) && !check_elevation::is_elevated().unwrap_or(true) {
        use std::ffi::OsString;
        use std::os::windows::prelude::OsStrExt;
        use windows::{
            core::PCWSTR,
            Win32::{
                Foundation::{HANDLE, HINSTANCE, HWND},
                System::Registry::HKEY,
                UI::Shell::{ShellExecuteExW, SEE_MASK_NOASYNC, SHELLEXECUTEINFOW},
                UI::WindowsAndMessaging::SW_NORMAL,
            },
        };
        let verb_wstr: [u16; 6] = [
            'r' as u16, 'u' as u16, 'n' as u16, 'a' as u16, 's' as u16, 0,
        ];
        let file_wstr = std::env::current_exe()
            .unwrap()
            .as_os_str()
            .encode_wide()
            .chain(Some(0))
            .collect::<Vec<_>>();
        let parameters_wstr = OsString::from("env up")
            .encode_wide()
            .chain(Some(0))
            .collect::<Vec<_>>();
        let mut sei = SHELLEXECUTEINFOW {
            cbSize: std::mem::size_of::<SHELLEXECUTEINFOW>() as u32,
            fMask: SEE_MASK_NOASYNC,
            hwnd: HWND::default(),
            lpVerb: PCWSTR(verb_wstr.as_ptr()),
            lpFile: PCWSTR(file_wstr.as_ptr()),
            lpParameters: PCWSTR(parameters_wstr.as_ptr()),
            lpDirectory: PCWSTR::null(),
            nShow: SW_NORMAL.0,
            hInstApp: HINSTANCE::default(),
            lpIDList: std::ptr::null_mut(),
            lpClass: PCWSTR::null(),
            hkeyClass: HKEY::default(),
            dwHotKey: 0,
            hProcess: HANDLE::default(),
            ..Default::default()
        };
        let result = unsafe { ShellExecuteExW(&mut sei) };
        if let Err(err) = result {
            let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                ui.set_output(format!("Failed to elevate: {}\n", err).into());
            });
            return;
        }
        std::process::exit(0);
    }

    let mut cmd: CommandBuilder = 'b: {
        #[cfg(unix)]
        if unsafe { libc::getuid() } != 0 {
            if which::which("lxqt-sudo").is_ok() {
                let mut cmd = CommandBuilder::new("lxqt-sudo");
                cmd.arg(PATH_OF_CLI.as_os_str());
                break 'b cmd;
            }
            if which::which("pkexec").is_ok() {
                let mut cmd = CommandBuilder::new("pkexec");
                cmd.arg(PATH_OF_CLI.as_os_str());
                break 'b cmd;
            }
        }
        break 'b CommandBuilder::new(PATH_OF_CLI.as_os_str());
    };
    cmd.arg("env");
    cmd.arg("up");
    let ui_handle_2 = ui_handle.clone();
    execute_cmd(ui_handle, cmd, true, move |status| {
        if status.success() {
            let _ = ui_handle_2.upgrade_in_event_loop(move |ui| {
                ui.set_env_status("OK".into());
            });
        } else {
            let _ = ui_handle_2.upgrade_in_event_loop(move |ui| {
                ui.set_env_status("Failed".into());
            });
        }
    });
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    if let Some(version_name) = CARGO_PKG_VERSION {
        ui.set_version_name(version_name.into());
    }

    let ui_handle = ui.as_weak();
    ui.on_env_up(move || {
        do_env_up(&ui_handle);
    });

    let ui_handle = ui.as_weak();
    ui.on_env_check(move || {
        do_env_check(&ui_handle);
    });

    let ui_handle = ui.as_weak();
    ui.on_act(move || {
        let mut cmd = CommandBuilder::new(PATH_OF_CLI.as_os_str());
        cmd.arg("tesseract");
        cmd.arg("--file");
        cmd.arg(ui_handle.unwrap().get_cproject_path().as_str());
        execute_cmd(&ui_handle, cmd, true, |_| {});
    });

    let ui_handle = ui.as_weak();
    ui.on_select_cproject(move || {
        let path = FileDialog::new()
            .add_filter("CProject File", &["cproject"])
            .pick_file();
        if let Some(path) = path {
            ui_handle
                .unwrap()
                .set_cproject_path(path.to_string_lossy().as_ref().into());
        }
    });

    let mut do_env_check_on_startup = true;

    if let Ok(cmd) = CommandParams::try_parse() {
        match cmd.command {
            command_params::Commands::Env { command } => match command {
                command_params::EnvCommands::Up {} => {
                    do_env_check_on_startup = false;
                    do_env_up(&ui.as_weak());
                }
            },
        }
    }

    if do_env_check_on_startup {
        do_env_check(&ui.as_weak());
    }

    ui.run()
}

fn execute_cmd<I>(ui_handle_ref: &Weak<AppWindow>, cmd: CommandBuilder, clear: bool, on_complete: I)
where
    I: FnOnce(&ExitStatus) + Send + 'static,
{
    if clear {
        let _ = ui_handle_ref.upgrade_in_event_loop(move |ui| {
            ui.set_output("".into());
        });
    }
    let pty_system = native_pty_system();
    let pair = pty_system.openpty(PtySize {
        rows: 256,
        cols: 256,
        pixel_width: 0,
        pixel_height: 0,
    });
    let pair = match pair {
        Ok(pair) => pair,
        Err(e) => {
            if let Some(ui) = ui_handle_ref.upgrade() {
                let mut output = ui.get_output();
                output.push_str(format!("// Failed to open pty: {}\n", e).as_str());
                ui.set_output(output);
            }
            on_complete(&ExitStatus::with_exit_code(1));
            return;
        }
    };

    let reader = match pair.master.try_clone_reader() {
        Ok(reader) => reader,
        Err(e) => {
            if let Some(ui) = ui_handle_ref.upgrade() {
                let mut output = ui.get_output();
                output.push_str(format!("// Failed to get reader: {}\n", e).as_str());
                ui.set_output(output);
            }
            on_complete(&ExitStatus::with_exit_code(1));
            return;
        }
    };

    let ui_handle = ui_handle_ref.clone();
    let join_handle = std::thread::spawn(move || {
        let mut lines_reader = BufReader::new(reader);
        loop {
            let mut next_line = String::new();
            match lines_reader.read_line(&mut next_line) {
                Ok(0) => break,
                Ok(_) => {
                    let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                        let mut output = ui.get_output();
                        output.push_str(strip_ansi_escapes::strip_str(next_line).as_str());
                        ui.set_output(output);
                    });
                }
                Err(e) => {
                    let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                        let mut output = ui.get_output();
                        output.push_str(format!("// Failed to read line: {}\n", e).as_str());
                        ui.set_output(output);
                    });
                    break;
                }
            }
        }
    });

    let mut child = match pair.slave.spawn_command(cmd) {
        Ok(child) => child,
        Err(e) => {
            if let Some(ui) = ui_handle_ref.upgrade() {
                let mut output = ui.get_output();
                output.push_str(format!("// Failed to spawn process: {}\n", e).as_str());
                ui.set_output(output);
            }

            on_complete(&ExitStatus::with_exit_code(1));
            return;
        }
    };

    let ui_handle = ui_handle_ref.clone();
    std::thread::spawn(move || match child.wait() {
        Ok(status) => {
            drop(child);
            drop(pair);
            drop(pty_system);
            let _ = join_handle.join();
            let cloned_status = status.clone();
            let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                let mut output = ui.get_output();
                output.push_str(
                    format!("// Process exited with status: {}\n", cloned_status).as_str(),
                );
                ui.set_output(output);
            });
            on_complete(&status);
        }
        Err(e) => {
            let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                let mut output = ui.get_output();
                output.push_str(format!("// Failed to wait: {}\n", e).as_str());
                ui.set_output(output);
            });
            on_complete(&ExitStatus::with_exit_code(1));
        }
    });
}
