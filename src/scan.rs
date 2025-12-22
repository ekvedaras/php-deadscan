mod discover;

use crate::scan::discover::discover_classes;
use std::path::PathBuf;

pub(crate) fn scan(path: &PathBuf, _json: &bool) {
    let class_map = discover_classes(path);
    dbg!(&class_map);
}
