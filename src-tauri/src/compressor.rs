use std::path::Path;
use std::process::Stdio;
use std::time::Instant;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use crate::state::{AppStateWrapper, CodecOption, OutputMode, AUDIO_BITRATE};

pub fn build_ffmpeg_args(
    _ffmpeg_path: &str,
    input_path: &str,
    output_path: &str,
    encoder: &str,
    bitrate_kbps: u32,
    codec: &CodecOption,
) -> Vec<String> {
    let mut args = vec![
        "-hide_banner".to_string(),
        "-i".to_string(),
        input_path.to_string(),
        "-c:v".to_string(),
        encoder.to_string(),
    ];

    // H.265 tag
    if *codec == CodecOption::H265 {
        args.push("-tag:v".to_string());
        args.push("hvc1".to_string());
    }

    args.extend_from_slice(&[
        "-preset".to_string(),
        "medium".to_string(),
        "-b:v".to_string(),
        format!("{}k", bitrate_kbps),
        "-c:a".to_string(),
        "aac".to_string(),
        "-b:a".to_string(),
        AUDIO_BITRATE.to_string(),
        "-movflags".to_string(),
        "+faststart".to_string(),
        "-map".to_string(),
        "0".to_string(),
        "-y".to_string(),
        "-progress".to_string(),
        "pipe:1".to_string(),
        "-nostats".to_string(),
        "-stats_period".to_string(),
        "0.2".to_string(),
        output_path.to_string(),
    ]);

    args
}

pub struct ProgressInfo {
    pub progress_fraction: f64,
    pub elapsed_secs: f64,
    pub remaining_secs: f64,
    pub estimated_total_secs: f64,
    pub is_complete: bool,
}

pub fn handle_progress_line(
    line: &str,
    duration_secs: f64,
    start_time: &Instant,
) -> Option<ProgressInfo> {
    let trimmed = line.trim();

    if trimmed.starts_with("progress=end") {
        return Some(ProgressInfo {
            progress_fraction: 1.0,
            elapsed_secs: start_time.elapsed().as_secs_f64(),
            remaining_secs: 0.0,
            estimated_total_secs: start_time.elapsed().as_secs_f64(),
            is_complete: true,
        });
    }

    let current_us = if trimmed.starts_with("out_time_us=") {
        trimmed
            .strip_prefix("out_time_us=")
            .and_then(|v| v.parse::<f64>().ok())
    } else if trimmed.starts_with("out_time_ms=") {
        trimmed
            .strip_prefix("out_time_ms=")
            .and_then(|v| v.parse::<f64>().ok())
            .map(|ms| ms * 1000.0)
    } else if trimmed.starts_with("out_time=") {
        trimmed
            .strip_prefix("out_time=")
            .and_then(|v| parse_out_time(v))
            .map(|secs| secs * 1_000_000.0)
    } else {
        None
    };

    if let Some(us) = current_us {
        if duration_secs > 0.0 {
            let current_secs = us / 1_000_000.0;
            let fraction = (current_secs / duration_secs).clamp(0.0, 1.0);
            let elapsed = start_time.elapsed().as_secs_f64();
            let estimated_total = if fraction > 0.001 {
                elapsed / fraction
            } else {
                0.0
            };
            let remaining = (estimated_total - elapsed).max(0.0);

            return Some(ProgressInfo {
                progress_fraction: fraction,
                elapsed_secs: elapsed,
                remaining_secs: remaining,
                estimated_total_secs: estimated_total,
                is_complete: false,
            });
        }
    }

    None
}

fn parse_out_time(s: &str) -> Option<f64> {
    // Format: HH:MM:SS.microseconds
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() == 3 {
        let h: f64 = parts[0].trim().parse().ok()?;
        let m: f64 = parts[1].trim().parse().ok()?;
        let secs: f64 = parts[2].trim().parse().ok()?;
        Some(h * 3600.0 + m * 60.0 + secs)
    } else {
        None
    }
}

pub async fn compress_file(
    app_handle: tauri::AppHandle,
    state_wrapper: AppStateWrapper,
    file_path: String,
    ffmpeg_path: String,
    duration_secs: f64,
) -> Result<(), String> {
    const MAX_ATTEMPTS: usize = 2;

    for attempt in 0..MAX_ATTEMPTS {
        let (encoder, bitrate, codec, output_path, output_mode) = {
            let state = state_wrapper.lock().await;
            let encoder = state.actual_encoder(&file_path);
            let bitrate = state.bitrate_kbps();
            let codec = state.codec.clone();
            let output_path = state.output_path_for(&file_path);
            let output_mode = state.output_mode.clone();
            (encoder, bitrate, codec, output_path, output_mode)
        };

        // Set current encoder text
        {
            let mut state = state_wrapper.lock().await;
            state.current_encoder_text = encoder.clone();
            state.current_file_name = Path::new(&file_path)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            state.current_file_progress = 0.0;
            state.current_file_elapsed = 0.0;
            state.current_file_remaining = 0.0;
            state.current_file_estimated_total = 0.0;
            state.in_progress.insert(file_path.clone());
            state.is_compressing = true;
        }

        let args = build_ffmpeg_args(
            &ffmpeg_path,
            &file_path,
            &output_path,
            &encoder,
            bitrate,
            &codec,
        );

        let start_time = Instant::now();

        let mut child = Command::new(&ffmpeg_path)
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("启动ffmpeg失败: {}", e))?;

        let stdout = child.stdout.take().ok_or("无法获取stdout")?;
        let mut reader = BufReader::new(stdout).lines();

        while let Ok(Some(line)) = reader.next_line().await {
            if let Some(progress) = handle_progress_line(&line, duration_secs, &start_time) {
                let mut state = state_wrapper.lock().await;
                state.current_file_progress = progress.progress_fraction;
                state.current_file_elapsed = progress.elapsed_secs;
                state.current_file_remaining = progress.remaining_secs;
                state.current_file_estimated_total = progress.estimated_total_secs;

                let frontend = state.to_frontend_state();
                drop(state);
                let _ = tauri::Emitter::emit(&app_handle, "state-updated", &frontend);
            }
        }

        let status = child
            .wait()
            .await
            .map_err(|e| format!("等待ffmpeg完成失败: {}", e))?;

        let mut state = state_wrapper.lock().await;
        state.in_progress.remove(&file_path);

        if !status.success() {
            // H.265 hardware failure fallback (only on first attempt)
            if attempt == 0 && codec == CodecOption::H265 && is_hardware_encoder(&encoder) {
                state.h265_software_fallback.insert(file_path.clone());
                state.add_log(&format!(
                    "硬件编码失败，将使用软件编码重试: {}",
                    Path::new(&file_path)
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                ));
                let _ = std::fs::remove_file(&output_path);
                drop(state);
                continue; // retry with software encoder
            }

            let _ = std::fs::remove_file(&output_path);
            return Err(format!("ffmpeg退出码: {}", status.code().unwrap_or(-1)));
        }

        // Handle output mode
        match output_mode {
            OutputMode::Overwrite => {
                std::fs::rename(&output_path, &file_path)
                    .map_err(|e| format!("替换文件失败: {}", e))?;
            }
            OutputMode::OutputFolder | OutputMode::Suffix => {}
        }

        let name = Path::new(&file_path)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        state.batch_completed += 1;
        state.last_completed_name = name.clone();
        state.current_file_progress = 1.0;
        state.processed_times.insert(
            file_path,
            chrono::Local::now().timestamp() as f64,
        );
        state.add_log(&format!("压缩完成: {}", name));

        return Ok(());
    }

    Err("超过最大重试次数".to_string())
}

fn is_hardware_encoder(encoder: &str) -> bool {
    encoder.contains("videotoolbox")
        || encoder.contains("nvenc")
        || encoder.contains("qsv")
        || encoder.contains("vaapi")
}
