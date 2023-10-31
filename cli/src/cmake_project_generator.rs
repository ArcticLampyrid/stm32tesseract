use regex::Regex;
use rhai::serde::to_dynamic;
use serde::{Deserialize, Serialize};
use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
struct FilterInfo {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    target_cpu: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    target_mcu: Vec<String>,
    #[serde(default)]
    overwrite: bool,
}
use crate::cproject_reader::CProjectInfo;

pub struct CMakeProjectGeneratorParams<'a> {
    pub info: &'a CProjectInfo,
}

fn get_cmake_project_template_path() -> io::Result<PathBuf> {
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
    path.push("templates");
    path.push("gcc");
    Ok(path)
}

fn match_filter(filter: &Vec<String>, target: &str) -> bool {
    for f in filter {
        let regex_expression = f.strip_prefix('!').unwrap_or(f.as_str());
        let regex = match Regex::new(regex_expression) {
            Ok(r) => r,
            Err(_) => continue,
        };
        if regex.is_match(target) {
            return !f.starts_with('!');
        }
    }
    true
}

fn copy_files_with_filter(
    context: &CMakeProjectGeneratorParams,
    source: &Path,
    dest: &Path,
) -> io::Result<()> {
    if !source.is_dir() {
        return Ok(());
    }

    if !dest.exists() {
        fs::create_dir_all(dest)?;
    }

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();

        let file_name = path.file_name();
        if file_name.is_none() {
            continue;
        }
        let file_name = file_name.unwrap();

        if path.is_dir() {
            let sub_dest = dest.join(file_name);
            copy_files_with_filter(context, &path, &sub_dest)?;
        } else if !file_name.to_string_lossy().ends_with(".filter") {
            let mut dest_path = dest.join(file_name);
            let filter_file_path = {
                let mut r = path.as_os_str().to_owned();
                r.push(".filter");
                PathBuf::from(r)
            };
            let mut overwrite = false;
            if filter_file_path.exists() {
                let filter_file = fs::read_to_string(&filter_file_path)?;
                let filter_info: FilterInfo = serde_json::from_str(&filter_file)?;
                overwrite = filter_info.overwrite;

                let mut include = true;
                include &= match_filter(
                    &filter_info.target_cpu,
                    context
                        .info
                        .target_cpu
                        .as_ref()
                        .map_or_else(|| "Unknown", |s| s.as_str()),
                );
                include &= match_filter(&filter_info.target_mcu, context.info.target_mcu.as_str());
                if !include {
                    continue;
                }
            }
            let file_content = if file_name.to_string_lossy().ends_with(".rhai") {
                let engine = rhai::Engine::new();
                let mut scope = rhai::Scope::new();
                if let Ok(info_dyn) = to_dynamic(context.info) {
                    scope.push_constant("info", info_dyn);
                }
                let script_result = engine.eval_file_with_scope::<String>(&mut scope, path.clone());
                let file_content = match script_result {
                    Ok(r) => r.to_string(),
                    Err(e) => format!("// Error: {}", e),
                };
                dest_path = dest_path.with_extension("");
                file_content
            } else {
                fs::read_to_string(&path)?
            };
            if dest_path.exists() && !overwrite {
                continue;
            }
            println!("Writing {}...", dest_path.display());
            fs::write(dest_path, file_content)?;
        }
    }

    Ok(())
}

pub fn generate_cmake_project(
    context: &CMakeProjectGeneratorParams,
    cproject_path: &Path,
) -> io::Result<()> {
    let template_path = get_cmake_project_template_path()?;
    let mut cmake_path = cproject_path.to_path_buf();
    cmake_path.pop();
    println!("Template used: {}", template_path.display());
    copy_files_with_filter(context, &template_path, &cmake_path)
}
