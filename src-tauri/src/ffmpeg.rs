use std::path::Path;
use std::process::Stdio;
use tokio::process::Command;

#[derive(Debug, Clone)]
pub struct FfmpegStatus {
    pub path: String,
    pub version_short: String,
    pub version_full: String,
    pub encoders: Vec<String>,
    pub encoder_error: String,
    pub hw_h264: bool,
    pub hw_h265: bool,
    pub hw_status_text: String,
    pub hw_status_hint: String,
}

pub fn resolve_ffmpeg_path(user_path: &str, app_data_dir: Option<&Path>) -> Option<String> {
    // 1. User-configured path
    if !user_path.is_empty() {
        let p = Path::new(user_path);
        if p.exists() {
            return Some(user_path.to_string());
        }
    }

    // 2. App data tools/ffmpeg
    if let Some(data_dir) = app_data_dir {
        let tool_path = data_dir.join("tools").join(ffmpeg_binary_name());
        if tool_path.exists() {
            return Some(tool_path.to_string_lossy().to_string());
        }
    }

    // 3. System paths
    #[cfg(target_os = "macos")]
    {
        let candidates = [
            "/opt/homebrew/bin/ffmpeg",
            "/usr/local/bin/ffmpeg",
            "/usr/bin/ffmpeg",
        ];
        for c in &candidates {
            if Path::new(c).exists() {
                return Some(c.to_string());
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        if let Ok(output) = std::process::Command::new("where")
            .arg("ffmpeg")
            .output()
        {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if let Some(line) = stdout.lines().next() {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        return Some(trimmed.to_string());
                    }
                }
            }
        }
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        let candidates = ["/usr/bin/ffmpeg", "/usr/local/bin/ffmpeg"];
        for c in &candidates {
            if Path::new(c).exists() {
                return Some(c.to_string());
            }
        }
    }

    None
}

fn ffmpeg_binary_name() -> &'static str {
    #[cfg(target_os = "windows")]
    { "ffmpeg.exe" }
    #[cfg(not(target_os = "windows"))]
    { "ffmpeg" }
}

pub fn parse_version_short(output: &str) -> String {
    if let Some(first_line) = output.lines().next() {
        let tokens: Vec<&str> = first_line.split_whitespace().collect();
        if tokens.len() >= 3 {
            return tokens[2].to_string();
        }
    }
    "unknown".to_string()
}

pub async fn detect_ffmpeg_status(ffmpeg_path: &str) -> FfmpegStatus {
    let mut status = FfmpegStatus {
        path: ffmpeg_path.to_string(),
        version_short: "未配置".to_string(),
        version_full: String::new(),
        encoders: Vec::new(),
        encoder_error: String::new(),
        hw_h264: false,
        hw_h265: false,
        hw_status_text: "硬件编码状态未知".to_string(),
        hw_status_hint: String::new(),
    };

    if ffmpeg_path.is_empty() {
        return status;
    }

    // Detect version
    match Command::new(ffmpeg_path)
        .arg("-version")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            status.version_full = stdout.clone();
            status.version_short = parse_version_short(&stdout);
        }
        Err(e) => {
            status.encoder_error = format!("无法运行ffmpeg: {}", e);
            return status;
        }
    }

    // Detect encoders
    match Command::new(ffmpeg_path)
        .arg("-encoders")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let filter_keywords = get_hw_filter_keywords();

            for line in stdout.lines() {
                let lower = line.to_lowercase();
                if filter_keywords.iter().any(|kw| lower.contains(kw)) {
                    status.encoders.push(line.trim().to_string());
                }
            }

            // Check hardware availability
            #[cfg(target_os = "macos")]
            {
                status.hw_h264 = stdout.contains("h264_videotoolbox");
                status.hw_h265 = stdout.contains("hevc_videotoolbox");
            }
            #[cfg(target_os = "windows")]
            {
                status.hw_h264 =
                    stdout.contains("h264_nvenc") || stdout.contains("h264_qsv");
                status.hw_h265 =
                    stdout.contains("hevc_nvenc") || stdout.contains("hevc_qsv");
            }
            #[cfg(not(any(target_os = "macos", target_os = "windows")))]
            {
                status.hw_h264 = stdout.contains("h264_nvenc") || stdout.contains("h264_vaapi");
                status.hw_h265 = stdout.contains("hevc_nvenc") || stdout.contains("hevc_vaapi");
            }

            if status.hw_h264 || status.hw_h265 {
                status.hw_status_text = "硬件编码可用".to_string();
                let mut parts = Vec::new();
                if status.hw_h264 {
                    parts.push("H.264");
                }
                if status.hw_h265 {
                    parts.push("H.265");
                }
                status.hw_status_hint = format!("支持: {}", parts.join(", "));
            } else {
                status.hw_status_text = "硬件编码不可用".to_string();
                status.hw_status_hint = "将使用软件编码".to_string();
            }
        }
        Err(e) => {
            status.encoder_error = format!("无法检测编码器: {}", e);
        }
    }

    status
}

fn get_hw_filter_keywords() -> Vec<&'static str> {
    #[cfg(target_os = "macos")]
    { vec!["videotoolbox"] }
    #[cfg(target_os = "windows")]
    { vec!["nvenc", "qsv"] }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    { vec!["nvenc", "qsv", "vaapi"] }
}

pub async fn probe_duration_seconds(ffmpeg_path: &str, file_path: &str) -> Option<f64> {
    let output = Command::new(ffmpeg_path)
        .args(["-i", file_path])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .ok()?;

    let stderr = String::from_utf8_lossy(&output.stderr);
    for line in stderr.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("Duration:") {
            // Duration: HH:MM:SS.xx,
            let parts: Vec<&str> = trimmed.splitn(2, ':').collect();
            if parts.len() >= 2 {
                let rest = parts[1].trim();
                let time_str = rest.split(',').next().unwrap_or("").trim();
                return parse_duration_str(time_str);
            }
        }
    }
    None
}

fn parse_duration_str(s: &str) -> Option<f64> {
    // Format: HH:MM:SS.xx
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

pub async fn download_ffmpeg(app_data_dir: &Path) -> Result<String, String> {
    let tools_dir = app_data_dir.join("tools");
    std::fs::create_dir_all(&tools_dir)
        .map_err(|e| format!("创建工具目录失败: {}", e))?;

    let url = get_download_url();
    let zip_path = tools_dir.join("ffmpeg.zip");

    let response = reqwest::get(url)
        .await
        .map_err(|e| format!("下载失败: {}", e))?;

    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("读取下载内容失败: {}", e))?;

    std::fs::write(&zip_path, &bytes)
        .map_err(|e| format!("保存文件失败: {}", e))?;

    // Unzip
    let file = std::fs::File::open(&zip_path)
        .map_err(|e| format!("打开zip失败: {}", e))?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| format!("解压失败: {}", e))?;

    let binary_name = ffmpeg_binary_name();
    let mut found = false;

    for i in 0..archive.len() {
        let mut entry = archive
            .by_index(i)
            .map_err(|e| format!("读取zip条目失败: {}", e))?;
        let entry_name = entry.name().to_string();
        if entry_name.ends_with(binary_name) && !entry_name.contains("__MACOSX") {
            let out_path = tools_dir.join(binary_name);
            let mut out_file = std::fs::File::create(&out_path)
                .map_err(|e| format!("创建文件失败: {}", e))?;
            std::io::copy(&mut entry, &mut out_file)
                .map_err(|e| format!("写入文件失败: {}", e))?;

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(&out_path, std::fs::Permissions::from_mode(0o755))
                    .map_err(|e| format!("设置权限失败: {}", e))?;
            }

            found = true;
            break;
        }
    }

    // Clean up zip
    let _ = std::fs::remove_file(&zip_path);

    if found {
        let result = tools_dir.join(binary_name);
        Ok(result.to_string_lossy().to_string())
    } else {
        Err("zip中未找到ffmpeg".to_string())
    }
}

fn get_download_url() -> &'static str {
    #[cfg(target_os = "macos")]
    { "https://evermeet.cx/ffmpeg/ffmpeg-6.1.1.zip" }
    #[cfg(target_os = "windows")]
    { "https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip" }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    { "https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip" }
}
