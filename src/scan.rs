mod discover;
mod reports;
mod usage;

use crate::scan::discover::{DiscoveredClass, discover_classes};
use crate::scan::usage::detect_usages;
pub(super) use reports::print_unused_classes_as_json;
pub(super) use reports::print_unused_classes_as_table;
use std::path::PathBuf;

pub(crate) fn scan(path: &PathBuf) -> Result<Vec<DiscoveredClass>, String> {
    let class_map = discover_classes(path)
        .map_err(|e| format!("Failed to discover classes: {}", e.to_string()))?;
    let usages =
        detect_usages(path).map_err(|e| format!("Failed to detect usages: {}", e.to_string()))?;

    Ok(class_map
        .iter()
        .filter(|(k, _path)| !usages.contains_key(*k))
        .map(|(_, v)| v.clone())
        .collect())
}
