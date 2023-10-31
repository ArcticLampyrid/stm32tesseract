#![windows_subsystem = "windows"]

use native_dialog::FileDialog;
use once_cell::sync::Lazy;
use portable_pty::{native_pty_system, CommandBuilder, ExitStatus, PtySize};
use slint::Weak;
use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
};

slint::include_modules!();

static PATH_OF_CLI: Lazy<PathBuf> = Lazy::new(|| {
    let mut path_of_cli = std::env::current_exe().expect("Failed to get current exe path");
    path_of_cli.pop();
    if cfg!(target_os = "windows") {
        path_of_cli.push("stm32tesseract.exe");
    } else {
        path_of_cli.push("stm32tesseract");
    }
    path_of_cli
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
    let mut cmd = CommandBuilder::new(PATH_OF_CLI.as_os_str());
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
            .show_open_single_file()
            .unwrap();
        if let Some(path) = path {
            ui_handle
                .unwrap()
                .set_cproject_path(path.to_string_lossy().as_ref().into());
        }
    });

    do_env_check(&ui.as_weak());

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
