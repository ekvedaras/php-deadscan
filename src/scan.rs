mod discover;
mod usage;

use crate::scan::discover::discover_classes;
use crate::scan::usage::detect_usages;
use std::path::PathBuf;

pub(crate) fn scan(path: &PathBuf, _json: &bool) {
    let class_map = discover_classes(path);
    dbg!(&class_map);
    let usages = detect_usages(path);
    dbg!(&usages);
}
