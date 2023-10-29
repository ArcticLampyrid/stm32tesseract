use std::path::Path;

use crate::{cproject_reader::CProjectInfo, simple_template};

pub fn expend_cmake_lists_template(cproject_path: &Path, info: &CProjectInfo) {
    let cmake_lists_template_path = {
        let mut path = cproject_path.to_path_buf();
        path.pop();
        path.push("CMakeLists_template.txt");
        path
    };
    let cmake_lists_path = {
        let mut path = cproject_path.to_path_buf();
        path.pop();
        path.push("CMakeLists.txt");
        path
    };
    let cmake_lists_prev = std::fs::read_to_string(cmake_lists_path.as_path())
        .ok()
        .unwrap_or_else(|| "".to_string());

    let mut vars = std::collections::HashMap::new();
    vars.insert(
        "templateWarning".to_string(),
        "THIS FILE IS AUTO GENERATED FROM THE TEMPLATE! DO NOT CHANGE!".to_string(),
    );
    vars.insert(
        "cmakeRequiredVersion".to_string(),
        "cmake_minimum_required(VERSION 3.25)\n".to_string(),
    );
    vars.insert("projectName".to_string(), info.project_name.clone());
    vars.insert(
        "mcpu".to_string(),
        info.target_cpu
            .clone()
            .unwrap_or_else(|| "Unknown".to_string()),
    );
    vars.insert("linkerScript".to_string(), info.linker_script.clone());
    vars.insert(
        "defines".to_string(),
        info.defined_symbols
            .iter()
            .map(|x| format!("-D{}", x))
            .collect::<Vec<_>>()
            .join(" "),
    );
    vars.insert("includes".to_string(), info.include_paths.join(" "));
    vars.insert(
        "sources".to_string(),
        info.source_entries
            .iter()
            .map(|x| format!("\"{}/*.*\"", x))
            .collect::<Vec<_>>()
            .join(" "),
    );
    let cmake_lists_template = std::fs::read_to_string(cmake_lists_template_path.as_path())
        .expect("Failed to read CMakeLists_template.txt");
    let cmake_lists = simple_template::expand_template(cmake_lists_template.as_str(), &vars);
    if cmake_lists.trim() != cmake_lists_prev.trim() {
        println!("CMakeLists.txt changed, writing to file");
        std::fs::write(cmake_lists_path.as_path(), cmake_lists.as_bytes())
            .expect("Failed to write CMakeLists.txt");
    } else {
        println!("CMakeLists.txt not changed");
    }
}
