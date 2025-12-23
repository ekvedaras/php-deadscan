use owo_colors::OwoColorize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;
use walkdir::WalkDir;

pub(super) fn discover_classes(path: &PathBuf) -> Result<HashMap<String, PathBuf>, String> {
    let mut classes = HashMap::new();

    for entry in WalkDir::new(std::env::current_dir().expect("cwd").join(path))
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().unwrap_or_default() == "php")
    {
        let file = File::open(entry.path())
            .map_err(|e| format!("Failed to read file {}", e.to_string()))?;
        let mut in_php_block = false;
        let mut in_comment_block = false;
        let mut in_now_doc_block = false;
        let mut now_doc_block_name = "".to_string();

        for line in io::BufReader::new(file)
            .lines()
            .filter_map(|l| l.ok())
            .map(|l| l.trim().to_string())
        {
            match line {
                line if (line.starts_with("class ")
                    || line.starts_with("final class ")
                    || line.starts_with("readonly class ")
                    || line.starts_with("final readonly class ")
                    || line.starts_with("abstract class ")
                    || line.starts_with("abstract readonly class ")
                    || line.starts_with("trait ")
                    || line.starts_with("interface "))
                    && in_php_block
                    && !in_comment_block
                    && !in_now_doc_block =>
                {
                    match line
                        .trim()
                        .split_whitespace()
                        .skip_while(|s| *s != "class" && *s != "trait" && *s != "interface")
                        .nth(1)
                    {
                        Some(class_name) => {
                            classes.insert(class_name.to_string(), entry.path().to_path_buf())
                        }
                        None => {
                            println!(
                                "{}: Unable to parse class name",
                                entry.path().display().red()
                            );
                            continue;
                        }
                    };
                }
                line if line.starts_with("<?php") => in_php_block = true,
                line if line.ends_with("?>") => in_php_block = false,
                line if line.starts_with("/*") => in_comment_block = true,
                line if line.starts_with("*/") => in_comment_block = false,
                line if line.starts_with("<<<") => match line.split_whitespace().nth(1) {
                    Some(name) => {
                        in_now_doc_block = true;
                        now_doc_block_name = name.to_string();
                    }
                    None => {
                        break;
                    }
                },
                line if in_now_doc_block && *line == format!("{};", now_doc_block_name) => {
                    in_now_doc_block = false;
                    now_doc_block_name = "".to_string();
                }
                _ => {}
            }
        }
    }

    Ok(classes)
}
