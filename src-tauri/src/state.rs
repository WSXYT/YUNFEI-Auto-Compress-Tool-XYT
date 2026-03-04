use chrono::{DateTime, Local, Duration, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

pub const APP_VERSION: &str = "1.0.2.51";
pub const APP_TITLE: &str = "YUNFEI自动压缩_1.0.2.51";
pub const APP_AUTHOR: &str = "制作者：摄影师云飞";
pub const APP_AUTHOR_LINK: &str = "https://space.bilibili.com/17519822";
pub const MAX_INPUT_FOLDER_COUNT: usize = 3;
pub const MAX_KEYWORD_COUNT: usize = 3;
pub const LOG_LIMIT: usize = 200;
pub const AUDIO_BITRATE: &str = "128k";

pub const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mov", "mkv", "avi", "m4v"];

pub const DEFAULT_SUFFIX: &str = "_压缩";

pub type AppStateWrapper = Arc<Mutex<AppState>>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TimeGateOption {
    Last24h,
    Last7d,
    CustomDate,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ScanIntervalOption {
    #[serde(rename = "1")]
    Min1,
    #[serde(rename = "5")]
    Min5,
    #[serde(rename = "15")]
    Min15,
    #[serde(rename = "60")]
    Min60,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum MountCheckIntervalOption {
    #[serde(rename = "3")]
    Min3,
    #[serde(rename = "5")]
    Min5,
    #[serde(rename = "10")]
    Min10,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum CodecOption {
    H264,
    H265,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum QualityPreset {
    SpaceSaving,
    Balanced,
    HighQuality,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum OutputMode {
    Overwrite,
    OutputFolder,
    Suffix,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueEntry {
    pub id: String,
    pub name: String,
    pub size_bytes: i64,
    pub estimated_bytes: Option<i64>,
}

impl QueueEntry {
    pub fn new(name: String, size_bytes: i64) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            size_bytes,
            estimated_bytes: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    // Settings (persisted)
    pub input_path: String,
    pub extra_input_paths: Vec<String>,
    pub output_path: String,
    pub include_subfolders: bool,
    pub time_gate: TimeGateOption,
    pub custom_date: Option<String>,
    pub scan_interval: ScanIntervalOption,
    pub immediate_compress: bool,
    pub codec: CodecOption,
    pub quality_preset: QualityPreset,
    pub output_mode: OutputMode,
    pub suffix_text: String,
    pub keywords: Vec<String>,
    pub launch_at_login: bool,
    pub keep_alive: bool,
    pub mount_monitor_enabled: bool,
    pub mount_interval: MountCheckIntervalOption,
    pub ffmpeg_path: String,

    // Runtime state
    pub scan_enabled: bool,
    pub status_text: String,
    pub mount_status: String,
    pub logs: Vec<String>,
    pub queue_items: Vec<QueueEntry>,
    pub batch_total: usize,
    pub batch_completed: usize,
    pub current_file_name: String,
    pub current_file_progress: f64,
    pub current_file_elapsed: f64,
    pub current_file_remaining: f64,
    pub current_file_estimated_total: f64,
    pub last_completed_name: String,
    pub ffmpeg_version_short: String,
    pub ffmpeg_version_full: String,
    pub ffmpeg_encoder_list: Vec<String>,
    pub ffmpeg_encoder_error: String,
    pub hardware_status_text: String,
    pub hardware_status_hint: String,
    pub current_encoder_text: String,
    pub hardware_available_h264: bool,
    pub hardware_available_h265: bool,
    pub is_downloading_ffmpeg: bool,
    pub last_scan: Option<DateTime<Local>>,
    pub pending_files: Vec<String>,
    pub processed_times: HashMap<String, f64>,
    pub last_seen_sizes: HashMap<String, i64>,
    pub in_progress: HashSet<String>,
    pub h265_software_fallback: HashSet<String>,
    pub is_compressing: bool,
    pub progress_buffer: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            input_path: String::new(),
            extra_input_paths: Vec::new(),
            output_path: String::new(),
            include_subfolders: false,
            time_gate: TimeGateOption::Last24h,
            custom_date: None,
            scan_interval: ScanIntervalOption::Min5,
            immediate_compress: false,
            codec: CodecOption::H265,
            quality_preset: QualityPreset::Balanced,
            output_mode: OutputMode::Overwrite,
            suffix_text: DEFAULT_SUFFIX.to_string(),
            keywords: vec!["带字幕".to_string()],
            launch_at_login: false,
            keep_alive: true,
            mount_monitor_enabled: true,
            mount_interval: MountCheckIntervalOption::Min5,
            ffmpeg_path: String::new(),

            scan_enabled: false,
            status_text: "已停止".to_string(),
            mount_status: "未检测".to_string(),
            logs: Vec::new(),
            queue_items: Vec::new(),
            batch_total: 0,
            batch_completed: 0,
            current_file_name: String::new(),
            current_file_progress: 0.0,
            current_file_elapsed: 0.0,
            current_file_remaining: 0.0,
            current_file_estimated_total: 0.0,
            last_completed_name: String::new(),
            ffmpeg_version_short: "未配置".to_string(),
            ffmpeg_version_full: String::new(),
            ffmpeg_encoder_list: Vec::new(),
            ffmpeg_encoder_error: String::new(),
            hardware_status_text: "硬件编码状态未知".to_string(),
            hardware_status_hint: String::new(),
            current_encoder_text: String::new(),
            hardware_available_h264: false,
            hardware_available_h265: false,
            is_downloading_ffmpeg: false,
            last_scan: None,
            pending_files: Vec::new(),
            processed_times: HashMap::new(),
            last_seen_sizes: HashMap::new(),
            in_progress: HashSet::new(),
            h265_software_fallback: HashSet::new(),
            is_compressing: false,
            progress_buffer: String::new(),
        }
    }
}

impl AppState {
    pub fn add_log(&mut self, message: &str) {
        let now = Local::now();
        let entry = format!("[{}] {}", now.format("%H:%M:%S"), message);
        self.logs.push(entry);
        if self.logs.len() > LOG_LIMIT {
            self.logs.remove(0);
        }
    }

    pub fn all_input_paths(&self) -> Vec<String> {
        let mut paths = Vec::new();
        if !self.input_path.is_empty() {
            paths.push(self.input_path.clone());
        }
        for p in &self.extra_input_paths {
            if !p.is_empty() {
                paths.push(p.clone());
            }
        }
        paths
    }

    pub fn bitrate_kbps(&self) -> u32 {
        match self.quality_preset {
            QualityPreset::SpaceSaving => 12000,
            QualityPreset::Balanced => 18000,
            QualityPreset::HighQuality => 25000,
        }
    }

    pub fn crf_value(&self) -> u32 {
        match self.codec {
            CodecOption::H264 => 23,
            CodecOption::H265 => 28,
        }
    }

    pub fn software_encoder(&self) -> &str {
        match self.codec {
            CodecOption::H264 => "libx264",
            CodecOption::H265 => "libx265",
        }
    }

    pub fn actual_encoder(&self, file_path: &str) -> String {
        match self.codec {
            CodecOption::H264 => {
                if self.hardware_available_h264 {
                    #[cfg(target_os = "macos")]
                    { return "h264_videotoolbox".to_string(); }
                    #[cfg(target_os = "windows")]
                    { return "h264_nvenc".to_string(); }
                    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
                    { return "libx264".to_string(); }
                }
                "libx264".to_string()
            }
            CodecOption::H265 => {
                if self.h265_software_fallback.contains(file_path) {
                    return "libx265".to_string();
                }
                if self.hardware_available_h265 {
                    #[cfg(target_os = "macos")]
                    { return "hevc_videotoolbox".to_string(); }
                    #[cfg(target_os = "windows")]
                    { return "hevc_nvenc".to_string(); }
                    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
                    { return "libx265".to_string(); }
                }
                "libx265".to_string()
            }
        }
    }

    pub fn output_path_for(&self, input: &str) -> String {
        let path = std::path::Path::new(input);
        let stem = path.file_stem().unwrap_or_default().to_string_lossy();
        let ext = path.extension().unwrap_or_default().to_string_lossy();
        let parent = path.parent().unwrap_or(std::path::Path::new(""));

        match self.output_mode {
            OutputMode::Overwrite => {
                let tmp = parent.join(format!("{}.tmp.{}", stem, ext));
                tmp.to_string_lossy().to_string()
            }
            OutputMode::OutputFolder => {
                if self.output_path.is_empty() {
                    parent
                        .join(format!("{}.{}", stem, ext))
                        .to_string_lossy()
                        .to_string()
                } else {
                    let out_dir = std::path::Path::new(&self.output_path);
                    out_dir
                        .join(format!("{}.{}", stem, ext))
                        .to_string_lossy()
                        .to_string()
                }
            }
            OutputMode::Suffix => {
                let suffix = if self.suffix_text.is_empty() {
                    DEFAULT_SUFFIX
                } else {
                    &self.suffix_text
                };
                parent
                    .join(format!("{}{}.{}", stem, suffix, ext))
                    .to_string_lossy()
                    .to_string()
            }
        }
    }

    pub fn cutoff_date(&self) -> Option<DateTime<Local>> {
        match self.time_gate {
            TimeGateOption::Last24h => Some(Local::now() - Duration::hours(24)),
            TimeGateOption::Last7d => Some(Local::now() - Duration::days(7)),
            TimeGateOption::CustomDate => {
                if let Some(ref date_str) = self.custom_date {
                    if let Ok(nd) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                        let dt = nd.and_hms_opt(0, 0, 0)?;
                        Some(DateTime::from_naive_utc_and_offset(dt, *Local::now().offset()))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }

    pub fn status_display(&self) -> String {
        format!(
            "{} · {} · v{}",
            self.status_text, self.mount_status, APP_VERSION
        )
    }

    pub fn queue_progress(&self) -> String {
        if self.batch_total == 0 {
            String::new()
        } else {
            format!("{}/{}", self.batch_completed, self.batch_total)
        }
    }

    pub fn to_frontend_state(&self) -> FrontendState {
        FrontendState {
            input_path: self.input_path.clone(),
            extra_input_paths: self.extra_input_paths.clone(),
            output_path: self.output_path.clone(),
            include_subfolders: self.include_subfolders,
            time_gate: self.time_gate.clone(),
            custom_date: self.custom_date.clone(),
            scan_interval: self.scan_interval.clone(),
            immediate_compress: self.immediate_compress,
            codec: self.codec.clone(),
            quality_preset: self.quality_preset.clone(),
            output_mode: self.output_mode.clone(),
            suffix_text: self.suffix_text.clone(),
            keywords: self.keywords.clone(),
            launch_at_login: self.launch_at_login,
            keep_alive: self.keep_alive,
            mount_monitor_enabled: self.mount_monitor_enabled,
            mount_interval: self.mount_interval.clone(),
            ffmpeg_path: self.ffmpeg_path.clone(),

            scan_enabled: self.scan_enabled,
            status_text: self.status_text.clone(),
            mount_status: self.mount_status.clone(),
            logs: self.logs.clone(),
            queue_items: self.queue_items.clone(),
            batch_total: self.batch_total,
            batch_completed: self.batch_completed,
            current_file_name: self.current_file_name.clone(),
            current_file_progress: self.current_file_progress,
            current_file_elapsed: self.current_file_elapsed,
            current_file_remaining: self.current_file_remaining,
            current_file_estimated_total: self.current_file_estimated_total,
            last_completed_name: self.last_completed_name.clone(),
            ffmpeg_version_short: self.ffmpeg_version_short.clone(),
            ffmpeg_version_full: self.ffmpeg_version_full.clone(),
            ffmpeg_encoder_list: self.ffmpeg_encoder_list.clone(),
            ffmpeg_encoder_error: self.ffmpeg_encoder_error.clone(),
            hardware_status_text: self.hardware_status_text.clone(),
            hardware_status_hint: self.hardware_status_hint.clone(),
            current_encoder_text: self.current_encoder_text.clone(),
            hardware_available_h264: self.hardware_available_h264,
            hardware_available_h265: self.hardware_available_h265,
            is_downloading_ffmpeg: self.is_downloading_ffmpeg,
            is_compressing: self.is_compressing,

            status_display: self.status_display(),
            queue_progress: self.queue_progress(),
            draft_keyword: String::new(),
            draft_suffix: self.suffix_text.clone(),
        }
    }

    pub fn scan_interval_seconds(&self) -> u64 {
        match self.scan_interval {
            ScanIntervalOption::Min1 => 60,
            ScanIntervalOption::Min5 => 300,
            ScanIntervalOption::Min15 => 900,
            ScanIntervalOption::Min60 => 3600,
        }
    }

    pub fn mount_interval_seconds(&self) -> u64 {
        match self.mount_interval {
            MountCheckIntervalOption::Min3 => 180,
            MountCheckIntervalOption::Min5 => 300,
            MountCheckIntervalOption::Min10 => 600,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrontendState {
    // Settings
    pub input_path: String,
    pub extra_input_paths: Vec<String>,
    pub output_path: String,
    pub include_subfolders: bool,
    pub time_gate: TimeGateOption,
    pub custom_date: Option<String>,
    pub scan_interval: ScanIntervalOption,
    pub immediate_compress: bool,
    pub codec: CodecOption,
    pub quality_preset: QualityPreset,
    pub output_mode: OutputMode,
    pub suffix_text: String,
    pub keywords: Vec<String>,
    pub launch_at_login: bool,
    pub keep_alive: bool,
    pub mount_monitor_enabled: bool,
    pub mount_interval: MountCheckIntervalOption,
    pub ffmpeg_path: String,

    // Runtime
    pub scan_enabled: bool,
    pub status_text: String,
    pub mount_status: String,
    pub logs: Vec<String>,
    pub queue_items: Vec<QueueEntry>,
    pub batch_total: usize,
    pub batch_completed: usize,
    pub current_file_name: String,
    pub current_file_progress: f64,
    pub current_file_elapsed: f64,
    pub current_file_remaining: f64,
    pub current_file_estimated_total: f64,
    pub last_completed_name: String,
    pub ffmpeg_version_short: String,
    pub ffmpeg_version_full: String,
    pub ffmpeg_encoder_list: Vec<String>,
    pub ffmpeg_encoder_error: String,
    pub hardware_status_text: String,
    pub hardware_status_hint: String,
    pub current_encoder_text: String,
    pub hardware_available_h264: bool,
    pub hardware_available_h265: bool,
    pub is_downloading_ffmpeg: bool,
    pub is_compressing: bool,

    // Computed
    pub status_display: String,
    pub queue_progress: String,
    pub draft_keyword: String,
    pub draft_suffix: String,
}
