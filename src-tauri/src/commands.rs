use serde_json::Value;
use tauri::{Manager, State};
use tauri_plugin_dialog::DialogExt;

use crate::ffmpeg;
use crate::scanner;
use crate::settings;
use crate::state::*;

#[tauri::command]
pub async fn get_state(state: State<'_, AppStateWrapper>) -> Result<FrontendState, String> {
    let s = state.lock().await;
    Ok(s.to_frontend_state())
}

#[tauri::command]
pub async fn update_settings(
    app_handle: tauri::AppHandle,
    state: State<'_, AppStateWrapper>,
    key: String,
    value: Value,
) -> Result<FrontendState, String> {
    let mut s = state.lock().await;

    match key.as_str() {
        "inputPath" => {
            if let Some(v) = value.as_str() {
                s.input_path = v.to_string();
            }
        }
        "outputPath" => {
            if let Some(v) = value.as_str() {
                s.output_path = v.to_string();
            }
        }
        "includeSubfolders" => {
            if let Some(v) = value.as_bool() {
                s.include_subfolders = v;
            }
        }
        "timeGate" => {
            if let Ok(v) = serde_json::from_value::<TimeGateOption>(value.clone()) {
                s.time_gate = v;
            }
        }
        "customDate" => {
            match value.as_str() {
                Some(v) if !v.is_empty() => {
                    s.custom_date = Some(v.to_string());
                }
                _ => {
                    s.custom_date = None;
                }
            }
        }
        "scanInterval" => {
            if let Ok(v) = serde_json::from_value::<ScanIntervalOption>(value.clone()) {
                s.scan_interval = v;
            }
        }
        "immediateCompress" => {
            if let Some(v) = value.as_bool() {
                s.immediate_compress = v;
            }
        }
        "codec" => {
            if let Ok(v) = serde_json::from_value::<CodecOption>(value.clone()) {
                s.codec = v;
            }
        }
        "qualityPreset" => {
            if let Ok(v) = serde_json::from_value::<QualityPreset>(value.clone()) {
                s.quality_preset = v;
            }
        }
        "outputMode" => {
            if let Ok(v) = serde_json::from_value::<OutputMode>(value.clone()) {
                s.output_mode = v;
            }
        }
        "suffixText" => {
            if let Some(v) = value.as_str() {
                s.suffix_text = v.to_string();
            }
        }
        "launchAtLogin" => {
            if let Some(v) = value.as_bool() {
                s.launch_at_login = v;
            }
        }
        "keepAlive" => {
            if let Some(v) = value.as_bool() {
                s.keep_alive = v;
            }
        }
        "mountMonitorEnabled" => {
            if let Some(v) = value.as_bool() {
                s.mount_monitor_enabled = v;
            }
        }
        "mountInterval" => {
            if let Ok(v) = serde_json::from_value::<MountCheckIntervalOption>(value.clone()) {
                s.mount_interval = v;
            }
        }
        "ffmpegPath" => {
            if let Some(v) = value.as_str() {
                s.ffmpeg_path = v.to_string();
            }
        }
        _ => {}
    }

    settings::save_setting(&app_handle, &key, &value);
    let frontend = s.to_frontend_state();
    Ok(frontend)
}

#[tauri::command]
pub async fn pick_folder(app_handle: tauri::AppHandle) -> Result<Option<String>, String> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    app_handle.dialog().file().pick_folder(move |folder| {
        let result = folder.map(|f| f.to_string());
        let _ = tx.send(result);
    });
    rx.await.map_err(|e| format!("对话框错误: {}", e))
}

#[tauri::command]
pub async fn pick_ffmpeg(app_handle: tauri::AppHandle) -> Result<Option<String>, String> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    app_handle.dialog().file().pick_file(move |file| {
        let result = file.map(|f| f.to_string());
        let _ = tx.send(result);
    });
    rx.await.map_err(|e| format!("对话框错误: {}", e))
}

#[tauri::command]
pub async fn start_scan(
    app_handle: tauri::AppHandle,
    state: State<'_, AppStateWrapper>,
) -> Result<FrontendState, String> {
    let mut s = state.lock().await;

    if s.input_path.is_empty() && s.extra_input_paths.is_empty() {
        return Err("请先设置输入目录".to_string());
    }

    s.scan_enabled = true;
    s.status_text = "监控中".to_string();
    s.add_log("开始监控扫描");

    // Do initial scan
    let found = scanner::scan_directories(&mut s);
    if !found.is_empty() {
        s.add_log(&format!("发现 {} 个待处理文件", found.len()));
        for f in &found {
            if !s.pending_files.contains(f) {
                s.pending_files.push(f.clone());
                s.last_seen_sizes.insert(
                    f.clone(),
                    std::fs::metadata(f).map(|m| m.len() as i64).unwrap_or(0),
                );
            }
        }
    }

    let frontend = s.to_frontend_state();
    let _ = tauri::Emitter::emit(&app_handle, "state-updated", &frontend);
    Ok(frontend)
}

#[tauri::command]
pub async fn stop_scan(
    app_handle: tauri::AppHandle,
    state: State<'_, AppStateWrapper>,
) -> Result<FrontendState, String> {
    let mut s = state.lock().await;
    s.scan_enabled = false;
    s.status_text = "已停止".to_string();
    s.add_log("停止监控扫描");

    let frontend = s.to_frontend_state();
    let _ = tauri::Emitter::emit(&app_handle, "state-updated", &frontend);
    Ok(frontend)
}

#[tauri::command]
pub async fn scan_once(
    app_handle: tauri::AppHandle,
    state: State<'_, AppStateWrapper>,
) -> Result<FrontendState, String> {
    let mut s = state.lock().await;
    s.add_log("手动扫描");

    let found = scanner::scan_directories(&mut s);
    if !found.is_empty() {
        s.add_log(&format!("发现 {} 个待处理文件", found.len()));
        for f in &found {
            if !s.pending_files.contains(f) {
                s.pending_files.push(f.clone());
                s.last_seen_sizes.insert(
                    f.clone(),
                    std::fs::metadata(f).map(|m| m.len() as i64).unwrap_or(0),
                );
            }
        }
    } else {
        s.add_log("未发现新文件");
    }

    s.last_scan = Some(chrono::Local::now());
    let frontend = s.to_frontend_state();
    let _ = tauri::Emitter::emit(&app_handle, "state-updated", &frontend);
    Ok(frontend)
}

#[tauri::command]
pub async fn apply_keywords(
    app_handle: tauri::AppHandle,
    state: State<'_, AppStateWrapper>,
    keywords: Vec<String>,
) -> Result<FrontendState, String> {
    let mut s = state.lock().await;
    let filtered: Vec<String> = keywords
        .into_iter()
        .filter(|k| !k.is_empty())
        .take(MAX_KEYWORD_COUNT)
        .collect();
    s.keywords = filtered;

    settings::save_setting(
        &app_handle,
        "keywords",
        &serde_json::json!(s.keywords),
    );

    let frontend = s.to_frontend_state();
    let _ = tauri::Emitter::emit(&app_handle, "state-updated", &frontend);
    Ok(frontend)
}

#[tauri::command]
pub async fn apply_suffix(
    app_handle: tauri::AppHandle,
    state: State<'_, AppStateWrapper>,
    suffix: String,
) -> Result<FrontendState, String> {
    let mut s = state.lock().await;
    s.suffix_text = suffix;

    settings::save_setting(
        &app_handle,
        "suffixText",
        &serde_json::json!(s.suffix_text),
    );

    let frontend = s.to_frontend_state();
    let _ = tauri::Emitter::emit(&app_handle, "state-updated", &frontend);
    Ok(frontend)
}

#[tauri::command]
pub async fn add_input_path(
    app_handle: tauri::AppHandle,
    state: State<'_, AppStateWrapper>,
    path: String,
) -> Result<FrontendState, String> {
    let mut s = state.lock().await;

    if s.extra_input_paths.len() >= MAX_INPUT_FOLDER_COUNT - 1 {
        return Err(format!("最多支持{}个输入目录", MAX_INPUT_FOLDER_COUNT));
    }

    if !s.extra_input_paths.contains(&path)
        && path != s.input_path
    {
        s.extra_input_paths.push(path);
        settings::save_setting(
            &app_handle,
            "extraInputPaths",
            &serde_json::json!(s.extra_input_paths),
        );
    }

    let frontend = s.to_frontend_state();
    let _ = tauri::Emitter::emit(&app_handle, "state-updated", &frontend);
    Ok(frontend)
}

#[tauri::command]
pub async fn remove_input_path(
    app_handle: tauri::AppHandle,
    state: State<'_, AppStateWrapper>,
    path: String,
) -> Result<FrontendState, String> {
    let mut s = state.lock().await;
    s.extra_input_paths.retain(|p| p != &path);

    settings::save_setting(
        &app_handle,
        "extraInputPaths",
        &serde_json::json!(s.extra_input_paths),
    );

    let frontend = s.to_frontend_state();
    let _ = tauri::Emitter::emit(&app_handle, "state-updated", &frontend);
    Ok(frontend)
}

#[tauri::command]
pub async fn get_logs(state: State<'_, AppStateWrapper>) -> Result<Vec<String>, String> {
    let s = state.lock().await;
    Ok(s.logs.clone())
}

#[tauri::command]
pub async fn get_queue(state: State<'_, AppStateWrapper>) -> Result<Vec<QueueEntry>, String> {
    let s = state.lock().await;
    Ok(s.queue_items.clone())
}

#[tauri::command]
pub async fn clear_queue(
    app_handle: tauri::AppHandle,
    state: State<'_, AppStateWrapper>,
) -> Result<FrontendState, String> {
    let mut s = state.lock().await;
    s.queue_items.clear();
    s.pending_files.clear();
    s.batch_total = 0;
    s.batch_completed = 0;
    s.add_log("已清空队列");

    let frontend = s.to_frontend_state();
    let _ = tauri::Emitter::emit(&app_handle, "state-updated", &frontend);
    Ok(frontend)
}

#[tauri::command]
pub async fn refresh_ffmpeg_status(
    app_handle: tauri::AppHandle,
    state: State<'_, AppStateWrapper>,
) -> Result<FrontendState, String> {
    let ffmpeg_path = {
        let s = state.lock().await;
        s.ffmpeg_path.clone()
    };

    let app_data_dir = app_handle.path().app_data_dir().ok();
    let resolved = ffmpeg::resolve_ffmpeg_path(&ffmpeg_path, app_data_dir.as_deref());

    let status = if let Some(ref path) = resolved {
        ffmpeg::detect_ffmpeg_status(path).await
    } else {
        ffmpeg::FfmpegStatus {
            path: String::new(),
            version_short: "未配置".to_string(),
            version_full: String::new(),
            encoders: Vec::new(),
            encoder_error: String::new(),
            hw_h264: false,
            hw_h265: false,
            hw_status_text: "硬件编码状态未知".to_string(),
            hw_status_hint: String::new(),
        }
    };

    let mut s = state.lock().await;
    if let Some(ref path) = resolved {
        s.ffmpeg_path = path.clone();
    }
    s.ffmpeg_version_short = status.version_short;
    s.ffmpeg_version_full = status.version_full;
    s.ffmpeg_encoder_list = status.encoders;
    s.ffmpeg_encoder_error = status.encoder_error;
    s.hardware_available_h264 = status.hw_h264;
    s.hardware_available_h265 = status.hw_h265;
    s.hardware_status_text = status.hw_status_text;
    s.hardware_status_hint = status.hw_status_hint;
    let ver = s.ffmpeg_version_short.clone();
    s.add_log(&format!("FFmpeg版本: {}", ver));

    let frontend = s.to_frontend_state();
    let _ = tauri::Emitter::emit(&app_handle, "state-updated", &frontend);
    Ok(frontend)
}

#[tauri::command]
pub async fn download_ffmpeg_cmd(
    app_handle: tauri::AppHandle,
    state: State<'_, AppStateWrapper>,
) -> Result<FrontendState, String> {
    {
        let mut s = state.lock().await;
        s.is_downloading_ffmpeg = true;
        s.add_log("开始下载FFmpeg...");
        let frontend = s.to_frontend_state();
        let _ = tauri::Emitter::emit(&app_handle, "state-updated", &frontend);
    }

    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("获取数据目录失败: {}", e))?;

    let result = ffmpeg::download_ffmpeg(&app_data_dir).await;

    let mut s = state.lock().await;
    s.is_downloading_ffmpeg = false;

    match result {
        Ok(path) => {
            s.ffmpeg_path = path;
            s.add_log("FFmpeg下载完成");
            settings::save_setting(
                &app_handle,
                "ffmpegPath",
                &serde_json::json!(s.ffmpeg_path),
            );
        }
        Err(e) => {
            s.add_log(&format!("FFmpeg下载失败: {}", e));
        }
    }

    let frontend = s.to_frontend_state();
    let _ = tauri::Emitter::emit(&app_handle, "state-updated", &frontend);
    Ok(frontend)
}
