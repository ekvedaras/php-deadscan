mod discover;
mod usage;

use crate::scan::discover::discover_classes;
use crate::scan::usage::detect_usages;
use std::collections::HashSet;
use std::path::PathBuf;

pub(crate) fn scan(path: &PathBuf, _json: &bool) -> Result<HashSet<String>, String> {
    let class_map = discover_classes(path)
        .map_err(|e| format!("Failed to discover classes: {}", e.to_string()))?;
    let usages =
        detect_usages(path).map_err(|e| format!("Failed to detect usages: {}", e.to_string()))?;

    Ok(class_map
        .keys()
        .filter(|k| !usages.contains_key(*k))
        .cloned()
        .collect())
}
