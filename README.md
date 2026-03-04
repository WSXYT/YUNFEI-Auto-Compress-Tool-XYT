<div align="center">
  <h1>云飞自动压缩工具 / YUNFEI Auto Compress Tool</h1>
  <p>
    <a href="#中文">🇨🇳 中文</a> | <a href="#english">🇬🇧 English</a>
  </p>
  <p>
    <img src="https://img.shields.io/badge/Tauri-2-blue?logo=tauri" alt="Tauri 2" />
    <img src="https://img.shields.io/badge/Vue-3-green?logo=vue.js" alt="Vue 3" />
    <img src="https://img.shields.io/badge/Rust-stable-orange?logo=rust" alt="Rust" />
    <img src="https://img.shields.io/badge/License-MIT-yellow" alt="MIT" />
    <img src="https://img.shields.io/badge/Platform-macOS%20%7C%20Windows-lightgrey" alt="macOS | Windows" />
  </p>
</div>

---

## 中文

### 项目简介

云飞自动压缩工具是一款跨平台桌面应用，专为内容创作者和视频工作室设计。它能够自动监控指定文件夹，通过关键词发现目标视频文件，并进行批量压缩处理，大幅提升视频后期工作效率。

**作者链接：**
- B站：[https://space.bilibili.com/17519822](https://space.bilibili.com/17519822)
- YouTube：[https://www.youtube.com/@SYS-YunFei](https://www.youtube.com/@SYS-YunFei)

### 功能特性

- **自动监控** — 实时监控指定文件夹，发现新视频文件自动加入压缩队列
- **关键词过滤** — 支持最多 3 个关键词，精准匹配目标视频文件
- **编码格式** — 支持 H.264 和 H.265 编码，灵活选择压缩质量
- **输出模式** — 多种输出方式：覆盖原文件、输出到指定文件夹、添加后缀
- **队列进度** — 可视化压缩队列，实时显示压缩进度和状态
- **FFmpeg 自动下载** — 自动检测和下载 FFmpeg，无需手动配置
- **后台运行 / 开机启动** — 支持后台静默运行和系统登录自启动
- **硬件编码加速** — 自动检测并利用 GPU 硬件加速编码
- **挂载监控** — 监控外部存储设备的挂载状态，确保文件夹可用
- **跨平台支持** — 支持 macOS 和 Windows

### 技术栈

| 层级 | 技术 |
|------|------|
| 框架 | [Tauri 2](https://tauri.app/) |
| 前端 | [Vue 3](https://vuejs.org/) + TypeScript + [Tailwind CSS](https://tailwindcss.com/) |
| 后端 | [Rust](https://www.rust-lang.org/) |

### 系统要求

- macOS 10.15+
- Windows 10+

### 快速开始

```bash
# 安装依赖
npm install

# 开发模式运行
npm run tauri dev

# 构建生产版本
npm run tauri build
```

### 项目结构

```
├── src/                    # Vue 前端源码
│   ├── components/         # UI 组件
│   ├── composables/        # 组合式函数
│   └── types/              # TypeScript 类型定义
├── src-tauri/              # Rust 后端源码
│   └── src/
│       ├── commands.rs     # Tauri 命令处理
│       ├── compressor.rs   # 视频压缩逻辑
│       ├── ffmpeg.rs       # FFmpeg 管理
│       ├── monitor.rs      # 文件夹监控
│       ├── scanner.rs      # 文件扫描
│       ├── settings.rs     # 配置持久化
│       ├── state.rs        # 应用状态管理
│       └── lib.rs          # 模块导出
├── docs/                   # 项目文档
└── package.json
```

### 许可证

本项目基于 [MIT 许可证](LICENSE) 开源。

---

## English

### Project Description

YUNFEI Auto Compress Tool is a cross-platform desktop application designed for content creators and video studios. It automatically monitors designated folders, discovers target video files through keyword matching, and performs batch compression — significantly improving post-production efficiency.

**Author Links:**
- Bilibili: [https://space.bilibili.com/17519822](https://space.bilibili.com/17519822)
- YouTube: [https://www.youtube.com/@SYS-YunFei](https://www.youtube.com/@SYS-YunFei)

### Features

- **Auto-Monitor** — Real-time monitoring of designated folders, automatically queuing new video files for compression
- **Keyword Filtering** — Up to 3 keywords for precise matching of target video files
- **Codec Support** — H.264 and H.265 encoding with flexible quality presets
- **Output Modes** — Multiple output options: overwrite original, output to folder, or add suffix
- **Queue Progress** — Visual compression queue with real-time progress and status
- **FFmpeg Auto-Download** — Automatic FFmpeg detection and download, no manual configuration needed
- **Background / Login Start** — Background operation and system login auto-start support
- **Hardware Encoding** — Automatic GPU hardware acceleration detection and utilization
- **Mount Monitoring** — Monitors external storage device mount status to ensure folder availability
- **Cross-Platform** — Supports macOS and Windows

### Tech Stack

| Layer | Technology |
|-------|------------|
| Framework | [Tauri 2](https://tauri.app/) |
| Frontend | [Vue 3](https://vuejs.org/) + TypeScript + [Tailwind CSS](https://tailwindcss.com/) |
| Backend | [Rust](https://www.rust-lang.org/) |

### System Requirements

- macOS 10.15+
- Windows 10+

### Quick Start

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

### Project Structure

```
├── src/                    # Vue frontend source
│   ├── components/         # UI components
│   ├── composables/        # Composition functions
│   └── types/              # TypeScript type definitions
├── src-tauri/              # Rust backend source
│   └── src/
│       ├── commands.rs     # Tauri command handlers
│       ├── compressor.rs   # Video compression logic
│       ├── ffmpeg.rs       # FFmpeg management
│       ├── monitor.rs      # Folder monitoring
│       ├── scanner.rs      # File scanning
│       ├── settings.rs     # Settings persistence
│       ├── state.rs        # Application state management
│       └── lib.rs          # Module exports
├── docs/                   # Documentation
└── package.json
```

### License

This project is open-sourced under the [MIT License](LICENSE).
