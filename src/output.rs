use crate::models::VSCodeInstance;

pub fn format_text(instances: &[VSCodeInstance], property_filter: Option<&str>) -> String {
    let mut lines = Vec::new();

    for instance in instances {
        if let Some(prop) = property_filter {
            if let Some(value) = get_property_value(instance, prop) {
                lines.push(value);
            }
        } else {
            lines.push(format!("installationPath: {}", instance.installation_path));
            lines.push(format!(
                "installationVersion: {}",
                instance.installation_version
            ));
            lines.push(format!("productPath: {}", instance.product_path));
            lines.push(format!("productId: {}", instance.product_id));
            lines.push(format!("isPrerelease: {}", instance.is_prerelease));
            lines.push(format!("displayName: {}", instance.display_name));
            lines.push(format!("extensionsPath: {}", instance.extensions_path));
            lines.push(format!("userDataPath: {}", instance.user_data_path));
            lines.push(String::new()); // Blank line between instances
        }
    }

    // Remove trailing empty line
    if lines.last().map(|s| s.is_empty()).unwrap_or(false) {
        lines.pop();
    }

    lines.join("\n")
}

pub fn format_json(instances: &[VSCodeInstance], property_filter: Option<&str>) -> String {
    if let Some(prop) = property_filter {
        let values: Vec<String> = instances
            .iter()
            .filter_map(|i| get_property_value(i, prop))
            .collect();
        serde_json::to_string_pretty(&values).unwrap_or_else(|_| "[]".to_string())
    } else {
        serde_json::to_string_pretty(instances).unwrap_or_else(|_| "[]".to_string())
    }
}

fn get_property_value(instance: &VSCodeInstance, property_name: &str) -> Option<String> {
    match property_name.to_lowercase().as_str() {
        "installationpath" => Some(instance.installation_path.clone()),
        "installationversion" => Some(instance.installation_version.clone()),
        "productpath" => Some(instance.product_path.clone()),
        "productid" => Some(instance.product_id.clone()),
        "isprerelease" => Some(instance.is_prerelease.to_string()),
        "displayname" => Some(instance.display_name.clone()),
        "extensionspath" => Some(instance.extensions_path.clone()),
        "userdatapath" => Some(instance.user_data_path.clone()),
        _ => None,
    }
}
