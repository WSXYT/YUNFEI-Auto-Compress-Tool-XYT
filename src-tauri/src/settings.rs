use serde_json::Value;
use std::collections::HashMap;
use tauri::Manager;
use tauri_plugin_store::StoreExt;

use crate::state::*;

pub fn load_settings(app_handle: &tauri::AppHandle, state: &mut AppState) {
    let store = match app_handle.store("settings.json") {
        Ok(s) => s,
        Err(_) => return,
    };

    if let Some(v) = store.get("inputPath") {
        if let Some(s) = v.as_str() {
            state.input_path = s.to_string();
        }
    }
    if let Some(v) = store.get("extraInputPaths") {
        if let Ok(paths) = serde_json::from_value::<Vec<String>>(v.clone()) {
            state.extra_input_paths = paths;
        }
    }
    if let Some(v) = store.get("outputPath") {
        if let Some(s) = v.as_str() {
            state.output_path = s.to_string();
        }
    }
    if let Some(v) = store.get("includeSubfolders") {
        if let Some(b) = v.as_bool() {
            state.include_subfolders = b;
        }
    }
    if let Some(v) = store.get("timeGate") {
        if let Ok(tg) = serde_json::from_value::<TimeGateOption>(v.clone()) {
            state.time_gate = tg;
        }
    }
    if let Some(v) = store.get("customDate") {
        if let Some(s) = v.as_str() {
            state.custom_date = Some(s.to_string());
        }
    }
    if let Some(v) = store.get("scanInterval") {
        if let Ok(si) = serde_json::from_value::<ScanIntervalOption>(v.clone()) {
            state.scan_interval = si;
        }
    }
    if let Some(v) = store.get("immediateCompress") {
        if let Some(b) = v.as_bool() {
            state.immediate_compress = b;
        }
    }
    if let Some(v) = store.get("codec") {
        if let Ok(c) = serde_json::from_value::<CodecOption>(v.clone()) {
            state.codec = c;
        }
    }
    if let Some(v) = store.get("qualityPreset") {
        if let Ok(qp) = serde_json::from_value::<QualityPreset>(v.clone()) {
            state.quality_preset = qp;
        }
    }
    if let Some(v) = store.get("outputMode") {
        if let Ok(om) = serde_json::from_value::<OutputMode>(v.clone()) {
            state.output_mode = om;
        }
    }
    if let Some(v) = store.get("suffixText") {
        if let Some(s) = v.as_str() {
            state.suffix_text = s.to_string();
        }
    }
    if let Some(v) = store.get("keywords") {
        if let Ok(kw) = serde_json::from_value::<Vec<String>>(v.clone()) {
            state.keywords = kw;
        }
    }
    if let Some(v) = store.get("launchAtLogin") {
        if let Some(b) = v.as_bool() {
            state.launch_at_login = b;
        }
    }
    if let Some(v) = store.get("keepAlive") {
        if let Some(b) = v.as_bool() {
            state.keep_alive = b;
        }
    }
    if let Some(v) = store.get("mountMonitorEnabled") {
        if let Some(b) = v.as_bool() {
            state.mount_monitor_enabled = b;
        }
    }
    if let Some(v) = store.get("mountInterval") {
        if let Ok(mi) = serde_json::from_value::<MountCheckIntervalOption>(v.clone()) {
            state.mount_interval = mi;
        }
    }
    if let Some(v) = store.get("ffmpegPath") {
        if let Some(s) = v.as_str() {
            state.ffmpeg_path = s.to_string();
        }
    }
}

pub fn save_setting(app_handle: &tauri::AppHandle, key: &str, value: &Value) {
    let store = match app_handle.store("settings.json") {
        Ok(s) => s,
        Err(_) => return,
    };
    store.set(key.to_string(), value.clone());
}

pub fn save_all_settings(app_handle: &tauri::AppHandle, state: &AppState) {
    let store = match app_handle.store("settings.json") {
        Ok(s) => s,
        Err(_) => return,
    };

    store.set("inputPath", serde_json::json!(state.input_path));
    store.set("extraInputPaths", serde_json::json!(state.extra_input_paths));
    store.set("outputPath", serde_json::json!(state.output_path));
    store.set("includeSubfolders", serde_json::json!(state.include_subfolders));
    store.set("timeGate", serde_json::to_value(&state.time_gate).unwrap_or_default());
    store.set("customDate", serde_json::json!(state.custom_date));
    store.set("scanInterval", serde_json::to_value(&state.scan_interval).unwrap_or_default());
    store.set("immediateCompress", serde_json::json!(state.immediate_compress));
    store.set("codec", serde_json::to_value(&state.codec).unwrap_or_default());
    store.set("qualityPreset", serde_json::to_value(&state.quality_preset).unwrap_or_default());
    store.set("outputMode", serde_json::to_value(&state.output_mode).unwrap_or_default());
    store.set("suffixText", serde_json::json!(state.suffix_text));
    store.set("keywords", serde_json::json!(state.keywords));
    store.set("launchAtLogin", serde_json::json!(state.launch_at_login));
    store.set("keepAlive", serde_json::json!(state.keep_alive));
    store.set("mountMonitorEnabled", serde_json::json!(state.mount_monitor_enabled));
    store.set("mountInterval", serde_json::to_value(&state.mount_interval).unwrap_or_default());
    store.set("ffmpegPath", serde_json::json!(state.ffmpeg_path));
}

pub fn load_processed_times(app_handle: &tauri::AppHandle) -> HashMap<String, f64> {
    let data_dir = match app_handle.path().app_data_dir() {
        Ok(d) => d,
        Err(_) => return HashMap::new(),
    };

    let path = data_dir.join("processed.json");
    if !path.exists() {
        return HashMap::new();
    }

    match std::fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => HashMap::new(),
    }
}

pub fn save_processed_times(app_handle: &tauri::AppHandle, times: &HashMap<String, f64>) {
    let data_dir = match app_handle.path().app_data_dir() {
        Ok(d) => d,
        Err(_) => return,
    };

    let _ = std::fs::create_dir_all(&data_dir);
    let path = data_dir.join("processed.json");

    if let Ok(content) = serde_json::to_string_pretty(times) {
        let _ = std::fs::write(&path, content);
    }
}
