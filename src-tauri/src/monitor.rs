use std::path::Path;

use crate::state::AppState;

pub fn check_mount_status(state: &mut AppState) -> String {
    let paths = state.all_input_paths();

    if paths.is_empty() {
        state.mount_status = "未配置输入目录".to_string();
        return state.mount_status.clone();
    }

    let mut all_ok = true;
    let mut missing = Vec::new();

    for p in &paths {
        if !Path::new(p).exists() {
            all_ok = false;
            missing.push(p.clone());
        }
    }

    if all_ok {
        state.mount_status = "所有目录已挂载".to_string();
    } else {
        state.mount_status = format!("{}个目录未挂载", missing.len());
    }

    state.mount_status.clone()
}

pub fn get_unmounted_paths(state: &AppState) -> Vec<String> {
    let paths = state.all_input_paths();
    let mut unmounted = Vec::new();
    for p in &paths {
        if !Path::new(p).exists() {
            unmounted.push(p.clone());
        }
    }
    unmounted
}
