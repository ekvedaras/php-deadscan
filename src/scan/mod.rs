use owo_colors::OwoColorize;
use std::path::PathBuf;
use walkdir::WalkDir;

pub(crate) fn scan(path: &PathBuf, json: &bool) {
    println!(
        "Scanning: {:?}, --json?={:?}",
        path.display().yellow(),
        json.yellow()
    );

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().ends_with(".php"))
    {
        println!("{:?}", entry.path().display().yellow());
    }
}
