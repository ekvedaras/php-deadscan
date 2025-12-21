use std::path::PathBuf;

pub(crate) fn scan(path: &PathBuf, json: &bool) {
    println!("Scanning: {:?}, --json?={:?}", path.display(), json);
}
