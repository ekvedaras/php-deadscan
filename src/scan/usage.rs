use owo_colors::OwoColorize;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;
use walkdir::WalkDir;

pub(super) fn detect_usages(path: &PathBuf) -> Result<HashMap<String, Vec<PathBuf>>, String> {
    let mut usages = HashMap::new();

    for entry in WalkDir::new(std::env::current_dir().expect("cwd").join(path))
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().unwrap_or_default() == "php")
    {
        match File::open(entry.path()) {
            Err(error) => {
                println!("Unable to open file: {}", error.red());
            }
            Ok(file) => {
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
                        line if (line.contains("new ")
                            || line.contains("extends ")
                            || line.contains("implements ")
                            || line.contains("use "))
                            && in_php_block
                            && !in_comment_block
                            && !in_now_doc_block =>
                        {
                            match line
                                .trim()
                                .split_whitespace()
                                .skip_while(|s| {
                                    *s != "new"
                                        && *s != "extends"
                                        && *s != "implements"
                                        && *s != "use"
                                })
                                .nth(1)
                            {
                                Some(class_name) => {
                                    usages
                                        .entry(class_name.trim_matches(['(', ')', ';']).to_string())
                                        .or_insert_with(Vec::new)
                                        .push(entry.path().to_path_buf());
                                }
                                None => {
                                    println!(
                                        "{}:{}: Unable to parse class name",
                                        entry.path().display(),
                                        line.red()
                                    );
                                    continue;
                                }
                            }
                        }
                        line if line.contains("::")
                            && in_php_block
                            && !in_comment_block
                            && !in_now_doc_block =>
                        {
                            match line
                                .trim()
                                .split_whitespace()
                                .skip_while(|s| !s.contains("::"))
                                .next()
                            {
                                Some(class_name) => {
                                    let extracted = class_name.split("::").nth(0);

                                    if let Some(extracted) = extracted {
                                        usages
                                            .entry(extracted.to_string())
                                            .or_insert_with(Vec::new)
                                            .push(entry.path().to_path_buf());
                                    }
                                }
                                None => {
                                    continue;
                                }
                            }
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
        }
    }

    Ok(usages)
}
