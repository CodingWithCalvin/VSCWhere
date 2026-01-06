use std::collections::HashSet;
use std::fs;
use std::path::Path;

use winreg::RegKey;
use winreg::enums::*;

use crate::models::{PackageInfo, ProductInfo, VSCodeInstance};

const UNINSTALL_KEY: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall";
const MICROSOFT_PUBLISHER: &str = "Microsoft Corporation";
const VSCODE_PATTERN: &str = "Microsoft Visual Studio Code";

pub fn discover_installations(include_prerelease: bool) -> Vec<VSCodeInstance> {
    let mut instances = Vec::new();
    let mut seen_paths: HashSet<String> = HashSet::new();

    // Search HKCU
    if let Ok(hkcu) = RegKey::predef(HKEY_CURRENT_USER).open_subkey(UNINSTALL_KEY) {
        search_registry(&hkcu, &mut instances, &mut seen_paths, include_prerelease);
    }

    // Search HKLM
    if let Ok(hklm) = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey(UNINSTALL_KEY) {
        search_registry(&hklm, &mut instances, &mut seen_paths, include_prerelease);
    }

    instances
}

fn search_registry(
    uninstall_key: &RegKey,
    instances: &mut Vec<VSCodeInstance>,
    seen_paths: &mut HashSet<String>,
    include_prerelease: bool,
) {
    for key_name in uninstall_key.enum_keys().filter_map(|k| k.ok()) {
        if let Ok(subkey) = uninstall_key.open_subkey(&key_name) {
            let publisher: String = subkey.get_value("Publisher").unwrap_or_default();
            let display_name: String = subkey.get_value("DisplayName").unwrap_or_default();
            let install_location: String = subkey.get_value("InstallLocation").unwrap_or_default();

            if !is_vscode_installation(&publisher, &display_name) || install_location.is_empty() {
                continue;
            }

            if let Some(instance) = build_instance(&install_location, &display_name) {
                if !include_prerelease && instance.is_prerelease {
                    continue;
                }

                let path_lower = instance.installation_path.to_lowercase();
                if !seen_paths.contains(&path_lower) {
                    seen_paths.insert(path_lower);
                    instances.push(instance);
                }
            }
        }
    }
}

fn is_vscode_installation(publisher: &str, display_name: &str) -> bool {
    publisher.eq_ignore_ascii_case(MICROSOFT_PUBLISHER)
        && display_name
            .to_lowercase()
            .contains(&VSCODE_PATTERN.to_lowercase())
}

fn build_instance(install_location: &str, registry_display_name: &str) -> Option<VSCodeInstance> {
    let install_path = Path::new(install_location);
    if !install_path.exists() {
        return None;
    }

    let product_json_path = install_path
        .join("resources")
        .join("app")
        .join("product.json");
    let package_json_path = install_path
        .join("resources")
        .join("app")
        .join("package.json");

    let product_info = read_json_file::<ProductInfo>(&product_json_path);
    let package_info = read_json_file::<PackageInfo>(&package_json_path);

    let version = package_info
        .as_ref()
        .and_then(|p| p.version.clone())
        .unwrap_or_else(|| "unknown".to_string());

    let quality = product_info
        .as_ref()
        .and_then(|p| p.quality.clone())
        .unwrap_or_else(|| determine_quality_from_display_name(registry_display_name));

    let is_prerelease = !quality.eq_ignore_ascii_case("stable");

    let display_name = product_info
        .as_ref()
        .and_then(|p| p.name_long.clone())
        .unwrap_or_else(|| registry_display_name.to_string());

    let product_path = find_executable(install_path)?;

    let user_profile = std::env::var("USERPROFILE").ok()?;
    let app_data = std::env::var("APPDATA").ok()?;

    let extensions_folder_name = product_info
        .as_ref()
        .and_then(|p| p.data_folder_name.clone())
        .unwrap_or_else(|| get_extensions_folder_name(&quality));

    let extensions_path = Path::new(&user_profile)
        .join(&extensions_folder_name)
        .join("extensions")
        .to_string_lossy()
        .to_string();

    let user_data_path = Path::new(&app_data)
        .join(get_user_data_folder_name(&quality))
        .to_string_lossy()
        .to_string();

    Some(VSCodeInstance {
        installation_path: install_location.to_string(),
        installation_version: version,
        product_path,
        product_id: quality,
        is_prerelease,
        display_name,
        extensions_path,
        user_data_path,
    })
}

fn read_json_file<T: serde::de::DeserializeOwned>(path: &Path) -> Option<T> {
    let content = fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

fn determine_quality_from_display_name(display_name: &str) -> String {
    let lower = display_name.to_lowercase();
    if lower.contains("insiders") {
        "insider".to_string()
    } else if lower.contains("exploration") {
        "exploration".to_string()
    } else {
        "stable".to_string()
    }
}

fn get_extensions_folder_name(quality: &str) -> String {
    match quality.to_lowercase().as_str() {
        "insider" => ".vscode-insiders".to_string(),
        "exploration" => ".vscode-exploration".to_string(),
        _ => ".vscode".to_string(),
    }
}

fn get_user_data_folder_name(quality: &str) -> String {
    match quality.to_lowercase().as_str() {
        "insider" => "Code - Insiders".to_string(),
        "exploration" => "Code - Exploration".to_string(),
        _ => "Code".to_string(),
    }
}

fn find_executable(install_path: &Path) -> Option<String> {
    let candidates = ["Code.exe", "Code - Insiders.exe", "Code - Exploration.exe"];

    for candidate in candidates {
        let exe_path = install_path.join(candidate);
        if exe_path.exists() {
            return Some(exe_path.to_string_lossy().to_string());
        }
    }

    None
}
