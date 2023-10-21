use std::path::Path;

use sxd_document::dom::Document;
use sxd_xpath::{evaluate_xpath, Value};

use crate::error::CProjectReaderError;
pub struct CProjectInfo {
    pub project_name: String,
    pub target_mcu: String,
    pub target_cpu: Option<String>,
    pub linker_script: String,
    pub defined_symbols: Vec<String>,
    pub include_paths: Vec<String>,
    pub source_entries: Vec<String>,
}

const XPATH_PROJECT_NAME: &str =
    "/cproject/storageModule[@moduleId='cdtBuildSystem']/project/@name[1]";
const XPATH_LINKER_SCRIPT: &str = "/cproject/storageModule[@moduleId='org.eclipse.cdt.core.settings']/cconfiguration[last()]/storageModule[@moduleId='cdtBuildSystem']/configuration[@artifactExtension='elf']/folderInfo/toolChain/tool/option[@superClass='com.st.stm32cube.ide.mcu.gnu.managedbuild.tool.c.linker.option.script']/@value[1]";
const XPATH_TARGET_MCU: &str = "/cproject/storageModule[@moduleId='org.eclipse.cdt.core.settings']/cconfiguration[last()]/storageModule[@moduleId='cdtBuildSystem']/configuration[@artifactExtension='elf']/folderInfo/toolChain/option[@superClass='com.st.stm32cube.ide.mcu.gnu.managedbuild.option.target_mcu']/@value[1]";
const XPATH_DEFINED_SYMBOLS: &str = "/cproject/storageModule[@moduleId='org.eclipse.cdt.core.settings']/cconfiguration[last()]/storageModule[@moduleId='cdtBuildSystem']/configuration[@artifactExtension='elf']/folderInfo/toolChain/tool/option[@superClass='com.st.stm32cube.ide.mcu.gnu.managedbuild.tool.c.compiler.option.definedsymbols']/listOptionValue/@value";
const XPATH_INCLUDE_PATHS: &str = "/cproject/storageModule[@moduleId='org.eclipse.cdt.core.settings']/cconfiguration[last()]/storageModule[@moduleId='cdtBuildSystem']/configuration[@artifactExtension='elf']/folderInfo/toolChain/tool/option[@superClass='com.st.stm32cube.ide.mcu.gnu.managedbuild.tool.c.compiler.option.includepaths']/listOptionValue/@value";
const XPATH_SOURCE_ENTRIES: &str = "/cproject/storageModule[@moduleId='org.eclipse.cdt.core.settings']/cconfiguration[last()]/storageModule[@moduleId='cdtBuildSystem']/configuration[@artifactExtension='elf']/sourceEntries/entry[@kind='sourcePath']/@name";

fn main_cpu_type_from_mcu_name(mcu_name: &str) -> Option<String> {
    if mcu_name.len() < 7 {
        return None;
    }
    match &mcu_name.to_ascii_uppercase().as_str()[..7] {
        "STM32C0" => Some("cortex-m0plus".to_string()),
        "STM32F0" => Some("cortex-m0".to_string()),
        "STM32F1" => Some("cortex-m3".to_string()),
        "STM32F2" => Some("cortex-m3".to_string()),
        "STM32F3" => Some("cortex-m4".to_string()),
        "STM32F4" => Some("cortex-m4".to_string()),
        "STM32F7" => Some("cortex-m7".to_string()),
        "STM32G0" => Some("cortex-m0plus".to_string()),
        "STM32G4" => Some("cortex-m4".to_string()),
        "STM32H5" => Some("cortex-m33".to_string()),
        "STM32H7" => Some("cortex-m7".to_string()),
        "STM32L0" => Some("cortex-m0plus".to_string()),
        "STM32L1" => Some("cortex-m3".to_string()),
        "STM32L4" => Some("cortex-m4".to_string()),
        "STM32L5" => Some("cortex-m33".to_string()),
        "STM32U5" => Some("cortex-m33".to_string()),
        "STM32WB" => Some("cortex-m0plus".to_string()),
        "STM32WL" => Some("cortex-m4".to_string()),
        _ => None,
    }
}

pub fn read_cproject(document: Document) -> Result<CProjectInfo, CProjectReaderError> {
    let project_name = evaluate_xpath(&document, XPATH_PROJECT_NAME)?.into_string();
    let target_mcu = evaluate_xpath(&document, XPATH_TARGET_MCU)?.into_string();

    let linker_script = evaluate_xpath(&document, XPATH_LINKER_SCRIPT)?
        .into_string()
        .split('/')
        .last()
        .unwrap_or_default()
        .to_string()
        .replace('{', "")
        .replace('}', "");

    let defined_symbols = match evaluate_xpath(&document, XPATH_DEFINED_SYMBOLS)? {
        Value::Nodeset(nodeset) => nodeset
            .into_iter()
            .map(|node| node.string_value())
            .collect(),
        _ => vec![],
    };
    let include_paths = match evaluate_xpath(&document, XPATH_INCLUDE_PATHS)? {
        Value::Nodeset(nodeset) => nodeset
            .into_iter()
            .map(|node| {
                let mut path = node.string_value();
                if path.starts_with("../") {
                    path = path.replacen("../", "", 1);
                }
                path
            })
            .collect(),
        _ => vec![],
    };
    let source_entries = match evaluate_xpath(&document, XPATH_SOURCE_ENTRIES)? {
        Value::Nodeset(nodeset) => nodeset
            .into_iter()
            .map(|node| node.string_value())
            .collect(),
        _ => vec![],
    };
    Ok(CProjectInfo {
        target_cpu: main_cpu_type_from_mcu_name(target_mcu.as_ref()),
        project_name,
        target_mcu,
        linker_script,
        defined_symbols,
        include_paths,
        source_entries,
    })
}

pub fn read_cproject_file<P>(path: P) -> Result<CProjectInfo, CProjectReaderError>
where
    P: AsRef<Path>,
{
    let content = std::fs::read_to_string(path)?;
    let package = sxd_document::parser::parse(content.as_ref())?;
    read_cproject(package.as_document())
}
