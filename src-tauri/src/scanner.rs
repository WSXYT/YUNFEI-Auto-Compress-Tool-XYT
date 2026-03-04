use chrono::{DateTime, Local};
use std::fs;
use std::path::Path;

use crate::state::{AppState, VIDEO_EXTENSIONS};

pub fn is_video_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| VIDEO_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

pub fn matches_keywords(name: &str, keywords: &[String]) -> bool {
    if keywords.is_empty() {
        return true;
    }
    keywords
        .iter()
        .any(|kw| !kw.is_empty() && name.contains(kw.as_str()))
}

pub fn scan_directories(state: &mut AppState) -> Vec<String> {
    let input_paths = state.all_input_paths();
    let cutoff = state.cutoff_date();
    let suffix = &state.suffix_text;
    let output_path = &state.output_path;
    let include_subfolders = state.include_subfolders;
    let keywords = state.keywords.clone();
    let output_mode = state.output_mode.clone();

    let mut found = Vec::new();

    for dir in &input_paths {
        let dir_path = Path::new(dir);
        if !dir_path.exists() || !dir_path.is_dir() {
            continue;
        }
        collect_files(
            dir_path,
            include_subfolders,
            &cutoff,
            suffix,
            output_path,
            &keywords,
            &output_mode,
            &state.processed_times,
            &state.in_progress,
            &mut found,
        );
    }

    found
}

fn collect_files(
    dir: &Path,
    recursive: bool,
    cutoff: &Option<DateTime<Local>>,
    suffix: &str,
    output_path: &str,
    keywords: &[String],
    output_mode: &crate::state::OutputMode,
    processed: &std::collections::HashMap<String, f64>,
    in_progress: &std::collections::HashSet<String>,
    found: &mut Vec<String>,
) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            // Skip output folder
            if !output_path.is_empty() {
                if let Ok(canon) = path.canonicalize() {
                    if let Ok(out_canon) = Path::new(output_path).canonicalize() {
                        if canon == out_canon {
                            continue;
                        }
                    }
                }
            }
            if recursive {
                collect_files(
                    &path,
                    recursive,
                    cutoff,
                    suffix,
                    output_path,
                    keywords,
                    output_mode,
                    processed,
                    in_progress,
                    found,
                );
            }
            continue;
        }

        if !is_video_file(&path) {
            continue;
        }

        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        // Skip suffix files
        if !suffix.is_empty() {
            let stem = path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            if stem.ends_with(suffix) {
                continue;
            }
        }

        // Keyword filter
        if !matches_keywords(&file_name, keywords) {
            continue;
        }

        let path_str = path.to_string_lossy().to_string();

        // Skip already processed
        if processed.contains_key(&path_str) {
            continue;
        }

        // Skip in progress
        if in_progress.contains(&path_str) {
            continue;
        }

        // Time gate
        if let Some(ref cutoff_dt) = cutoff {
            if let Ok(metadata) = fs::metadata(&path) {
                if let Ok(modified) = metadata.modified() {
                    let mod_dt: DateTime<Local> = modified.into();
                    if mod_dt < *cutoff_dt {
                        continue;
                    }
                }
            }
        }

        found.push(path_str);
    }
}

pub fn check_file_stability(state: &mut AppState) -> Vec<String> {
    let mut stable = Vec::new();
    let mut changed = Vec::new();

    for file in &state.pending_files {
        let path = Path::new(file);
        if let Ok(metadata) = fs::metadata(path) {
            let current_size = metadata.len() as i64;
            if let Some(&last_size) = state.last_seen_sizes.get(file) {
                if current_size == last_size {
                    stable.push(file.clone());
                } else {
                    changed.push((file.clone(), current_size));
                }
            } else {
                changed.push((file.clone(), current_size));
            }
        }
    }

    // Update sizes for changed files
    for (file, size) in &changed {
        state.last_seen_sizes.insert(file.clone(), *size);
    }

    // Set sizes for stable files that weren't seen before
    for file in &stable {
        if !state.last_seen_sizes.contains_key(file) {
            if let Ok(metadata) = fs::metadata(file) {
                state.last_seen_sizes.insert(file.clone(), metadata.len() as i64);
            }
        }
    }

    stable
}
