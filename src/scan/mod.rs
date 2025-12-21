use std::path::PathBuf;
use owo_colors::OwoColorize;

pub(crate) fn scan(path: &PathBuf, json: &bool) {
    println!("Scanning: {:?}, --json?={:?}", path.display().yellow(), json.yellow());
}
