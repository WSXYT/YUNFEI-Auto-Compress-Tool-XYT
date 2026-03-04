pub mod commands;
pub mod compressor;
pub mod ffmpeg;
pub mod monitor;
pub mod scanner;
pub mod settings;
pub mod state;

use state::{AppState, AppStateWrapper, APP_TITLE, APP_VERSION};
use std::sync::Arc;
use tauri::{Emitter, Manager};
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::TrayIconBuilder;
use tauri_plugin_autostart::MacosLauncher;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state_wrapper: AppStateWrapper = Arc::new(tokio::sync::Mutex::new(AppState::default()));
    let state_wrapper_close = state_wrapper.clone();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_store::Builder::default().build())
        .manage(state_wrapper.clone())
        .setup(move |app| {
            let app_handle = app.handle().clone();

            // Load settings and processed times
            {
                let state_wrapper = state_wrapper.clone();
                let app_handle = app_handle.clone();
                tauri::async_runtime::block_on(async {
                    let mut s = state_wrapper.lock().await;
                    settings::load_settings(&app_handle, &mut s);
                    s.processed_times = settings::load_processed_times(&app_handle);
                    s.add_log(&format!("YUNFEI自动压缩 v{} 已启动", APP_VERSION));
                });
            }

            // Resolve ffmpeg
            {
                let state_wrapper = state_wrapper.clone();
                let app_handle = app_handle.clone();
                tauri::async_runtime::spawn(async move {
                    let ffmpeg_path = {
                        let s = state_wrapper.lock().await;
                        s.ffmpeg_path.clone()
                    };

                    let app_data_dir = app_handle.path().app_data_dir().ok();
                    let resolved =
                        ffmpeg::resolve_ffmpeg_path(&ffmpeg_path, app_data_dir.as_deref());

                    if let Some(ref path) = resolved {
                        let status = ffmpeg::detect_ffmpeg_status(path).await;
                        let mut s = state_wrapper.lock().await;
                        s.ffmpeg_path = path.clone();
                        s.ffmpeg_version_short = status.version_short;
                        s.ffmpeg_version_full = status.version_full;
                        s.ffmpeg_encoder_list = status.encoders;
                        s.ffmpeg_encoder_error = status.encoder_error;
                        s.hardware_available_h264 = status.hw_h264;
                        s.hardware_available_h265 = status.hw_h265;
                        s.hardware_status_text = status.hw_status_text;
                        s.hardware_status_hint = status.hw_status_hint;
                        let ver = s.ffmpeg_version_short.clone();
                        s.add_log(&format!("FFmpeg: {}", ver));
                    }
                });
            }

            // System tray
            build_system_tray(app)?;

            // Periodic scan timer
            {
                let state_wrapper_scan = app.state::<AppStateWrapper>().inner().clone();
                let app_handle_scan = app_handle.clone();
                tauri::async_runtime::spawn(async move {
                    loop {
                        let interval = {
                            let s = state_wrapper_scan.lock().await;
                            s.scan_interval_seconds()
                        };
                        tokio::time::sleep(tokio::time::Duration::from_secs(interval)).await;

                        let should_scan = {
                            let s = state_wrapper_scan.lock().await;
                            s.scan_enabled
                        };

                        if should_scan {
                            let (found, immediate, ffmpeg_path) = {
                                let mut s = state_wrapper_scan.lock().await;
                                let found = scanner::scan_directories(&mut s);
                                let new_count = found
                                    .iter()
                                    .filter(|f| !s.pending_files.contains(f))
                                    .count();

                                if new_count > 0 {
                                    s.add_log(&format!("扫描发现 {} 个新文件", new_count));
                                    for f in &found {
                                        if !s.pending_files.contains(f) {
                                            s.pending_files.push(f.clone());
                                            s.last_seen_sizes.insert(
                                                f.clone(),
                                                std::fs::metadata(f)
                                                    .map(|m| m.len() as i64)
                                                    .unwrap_or(0),
                                            );
                                        }
                                    }
                                }
                                s.last_scan = Some(chrono::Local::now());

                                let immediate = s.immediate_compress;
                                let ffmpeg_path = s.ffmpeg_path.clone();
                                (found, immediate, ffmpeg_path)
                            };

                            // Emit state
                            {
                                let s = state_wrapper_scan.lock().await;
                                let frontend = s.to_frontend_state();
                                let _ = app_handle_scan.emit("state-updated", &frontend);
                            }

                            // Start compression if immediate mode
                            if immediate && !found.is_empty() && !ffmpeg_path.is_empty() {
                                run_compression_batch(
                                    app_handle_scan.clone(),
                                    state_wrapper_scan.clone(),
                                )
                                .await;
                            }
                        }
                    }
                });
            }

            // Periodic mount check timer
            {
                let state_wrapper_mount = app.state::<AppStateWrapper>().inner().clone();
                let app_handle_mount = app_handle.clone();
                tauri::async_runtime::spawn(async move {
                    loop {
                        let (interval, enabled) = {
                            let s = state_wrapper_mount.lock().await;
                            (s.mount_interval_seconds(), s.mount_monitor_enabled)
                        };
                        tokio::time::sleep(tokio::time::Duration::from_secs(interval)).await;

                        if enabled {
                            let unmounted = {
                                let mut s = state_wrapper_mount.lock().await;
                                monitor::check_mount_status(&mut s);
                                monitor::get_unmounted_paths(&s)
                            };

                            if !unmounted.is_empty() {
                                let mut s = state_wrapper_mount.lock().await;
                                s.add_log(&format!(
                                    "检测到 {} 个目录未挂载",
                                    unmounted.len()
                                ));
                                let frontend = s.to_frontend_state();
                                let _ = app_handle_mount.emit("state-updated", &frontend);
                            }
                        }
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_state,
            commands::update_settings,
            commands::pick_folder,
            commands::pick_ffmpeg,
            commands::start_scan,
            commands::stop_scan,
            commands::scan_once,
            commands::apply_keywords,
            commands::apply_suffix,
            commands::add_input_path,
            commands::remove_input_path,
            commands::get_logs,
            commands::get_queue,
            commands::clear_queue,
            commands::refresh_ffmpeg_status,
            commands::download_ffmpeg_cmd,
        ])
        .on_window_event(move |window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let keep_alive = tauri::async_runtime::block_on(async {
                    let s = state_wrapper_close.lock().await;
                    s.keep_alive
                });

                if keep_alive {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn build_system_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let current_task = MenuItemBuilder::with_id("current_task", "当前任务: 无")
        .enabled(false)
        .build(app)?;
    let queue_info = MenuItemBuilder::with_id("queue_info", "队列: 0")
        .enabled(false)
        .build(app)?;
    let recent_log = MenuItemBuilder::with_id("recent_log", "最近日志: 无")
        .enabled(false)
        .build(app)?;
    let show_window = MenuItemBuilder::with_id("show_window", "打开主窗口").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "退出").build(app)?;

    let menu = MenuBuilder::new(app)
        .item(&current_task)
        .item(&queue_info)
        .item(&recent_log)
        .separator()
        .item(&show_window)
        .item(&quit)
        .build()?;

    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .tooltip(APP_TITLE)
        .on_menu_event(move |app_handle, event| {
            match event.id().as_ref() {
                "show_window" => {
                    if let Some(win) = app_handle.get_webview_window("main") {
                        let _ = win.show();
                        let _ = win.set_focus();
                    }
                }
                "quit" => {
                    // Save processed times before quit
                    let state_wrapper = app_handle.state::<AppStateWrapper>();
                    let app_handle_clone = app_handle.clone();
                    tauri::async_runtime::block_on(async {
                        let s = state_wrapper.lock().await;
                        settings::save_processed_times(&app_handle_clone, &s.processed_times);
                        settings::save_all_settings(&app_handle_clone, &s);
                    });
                    app_handle.exit(0);
                }
                _ => {}
            }
        })
        .build(app)?;

    Ok(())
}

async fn run_compression_batch(app_handle: tauri::AppHandle, state_wrapper: AppStateWrapper) {
    let files_to_process: Vec<String> = {
        let mut s = state_wrapper.lock().await;

        if s.is_compressing {
            return;
        }

        // Check file stability first
        let stable = scanner::check_file_stability(&mut s);
        if stable.is_empty() {
            return;
        }

        s.is_compressing = true;

        // Build queue entries
        let mut entries = Vec::new();
        for f in &stable {
            let name = std::path::Path::new(f)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let size = std::fs::metadata(f).map(|m| m.len() as i64).unwrap_or(0);
            entries.push(state::QueueEntry::new(name, size));
        }

        s.queue_items = entries;
        s.batch_total = stable.len();
        s.batch_completed = 0;
        s.status_text = "压缩中".to_string();

        stable
    };

    let ffmpeg_path = {
        let s = state_wrapper.lock().await;
        s.ffmpeg_path.clone()
    };

    for file in &files_to_process {
        // Probe duration
        let duration = ffmpeg::probe_duration_seconds(&ffmpeg_path, file)
            .await
            .unwrap_or(0.0);

        let result = compressor::compress_file(
            app_handle.clone(),
            state_wrapper.clone(),
            file.clone(),
            ffmpeg_path.clone(),
            duration,
        )
        .await;

        let mut s = state_wrapper.lock().await;
        match result {
            Ok(()) => {
                // Remove from pending
                s.pending_files.retain(|f| f != file);
            }
            Err(e) => {
                s.add_log(&format!("压缩失败: {} - {}", file, e));
                s.pending_files.retain(|f| f != file);
            }
        }

        // Save progress
        settings::save_processed_times(&app_handle, &s.processed_times);

        let frontend = s.to_frontend_state();
        let _ = app_handle.emit("state-updated", &frontend);
    }

    let mut s = state_wrapper.lock().await;
    s.is_compressing = false;
    s.status_text = if s.scan_enabled {
        "监控中".to_string()
    } else {
        "已停止".to_string()
    };
    s.current_file_name = String::new();
    s.current_file_progress = 0.0;

    let frontend = s.to_frontend_state();
    let _ = app_handle.emit("state-updated", &frontend);
}
