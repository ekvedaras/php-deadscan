mod discover;
mod usage;

use crate::scan::discover::discover_classes;
use crate::scan::usage::detect_usages;
use std::collections::HashMap;
use std::path::PathBuf;

pub(crate) fn scan(path: &PathBuf, _json: &bool) -> Result<HashMap<String, PathBuf>, String> {
    let class_map = discover_classes(path)
        .map_err(|e| format!("Failed to discover classes: {}", e.to_string()))?;
    let usages =
        detect_usages(path).map_err(|e| format!("Failed to detect usages: {}", e.to_string()))?;

    Ok(class_map
        .iter()
        .filter(|(k, _path)| !usages.contains_key(*k))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect())
}
