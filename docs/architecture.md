# 架构

## 系统概述

YUNFEI Auto Compress Tool 基于 **Tauri 2** 构建，结合了 Rust 后端和基于 Web 的前端。架构遵循清晰的关注点分离原则：

```
┌─────────────────────────────────┐
│         Vue 3 Frontend          │
│   (WebView / HTML + CSS + JS)   │
├─────────────────────────────────┤
│        Tauri IPC Bridge         │
├─────────────────────────────────┤
│         Rust Backend            │
│  (State, FFmpeg, Compression)   │
├─────────────────────────────────┤
│     OS / File System / GPU      │
└─────────────────────────────────┘
```

- **前端** 负责渲染用户界面并捕获用户交互。
- **Tauri IPC** 将前端调用桥接到 Rust 命令处理器。
- **后端** 管理状态、文件扫描、视频压缩和系统集成。

## 前端架构

### 入口点

`src/main.ts` 创建 Vue 应用并将其挂载到 `index.html` 中的 `#app`。

### 组件

所有组件位于 `src/components/`：

| 组件 | 职责 |
|-----------|---------------|
| `HeaderSection.vue` | 应用标题和作者信息 |
| `MonitorSection.vue` | 输入文件夹管理（主文件夹 + 2 个额外文件夹） |
| `ScanSection.vue` | 关键词过滤（最多 3 个关键词） |
| `CompressSection.vue` | 编解码器和质量预设选择（H.264/H.265） |
| `OutputSection.vue` | 输出模式配置（覆盖、文件夹、后缀） |
| `RuntimeSection.vue` | FFmpeg 管理、队列状态、压缩进度 |
| `LogSection.vue` | 最近日志和更新历史 |
| `SectionCard.vue` | 可复用的 UI 区块卡片容器 |
| `HelpDialog.vue` | 帮助弹窗 |
| `DonateDialog.vue` | 捐赠弹窗 |

### 组合式函数

`src/composables/useAppState.ts` 是核心状态管理组合式函数。它提供：

- 与 Rust 后端同步的响应式应用状态
- 方法：`init()`、`updateSetting()`、`startScan()`、`stopScan()`、`pickFolder()`、`pickFfmpeg()`、`refreshFfmpeg()`、`clearQueue()` 等
- 后端发出事件的监听器（进度更新、扫描结果）

### 类型定义

`src/types/index.ts` 定义了共享状态模型的 TypeScript 接口，与 Rust 的 `AppState` 结构体保持一致。

## 后端架构

### 模块概述

所有 Rust 源文件位于 `src-tauri/src/`：

| 模块 | 文件 | 职责 |
|--------|------|---------------|
| **状态** | `state.rs` | 核心 `AppState` 结构体及相关枚举（`CodecOption`、`OutputMode`、`TimeGateOption` 等） |
| **命令** | `commands.rs` | Tauri `#[command]` 处理器 —— 由前端调用的 IPC 入口点 |
| **FFmpeg** | `ffmpeg.rs` | FFmpeg 二进制文件检测、路径解析、硬件加速检查、自动下载 |
| **扫描器** | `scanner.rs` | 扫描目录中与关键词匹配的视频文件；按日期过滤 |
| **压缩器** | `compressor.rs` | 构建用于压缩的 FFmpeg 参数（编解码器、比特率、音频设置） |
| **监控器** | `monitor.rs` | 验证输入目录的可用性；检查文件夹挂载状态 |
| **设置** | `settings.rs` | 从持久化 JSON 存储中加载/保存应用配置 |
| **库** | `lib.rs` | 模块声明和 Tauri 插件/命令注册 |

### 状态管理

`AppState` 是唯一的数据源，封装在 `Arc<Mutex<>>` 中并在所有命令处理器之间共享。关键字段包括：

- 输入路径（主文件夹 + 额外文件夹）
- 关键词和扫描设置
- 编解码器、比特率和输出模式配置
- FFmpeg 路径和硬件加速状态
- 压缩队列和进度

### FFmpeg 集成

`ffmpeg` 模块处理以下功能：

1. **检测** —— 搜索系统 PATH 和常见安装路径
2. **验证** —— 验证二进制文件是否可用并检查版本
3. **硬件加速** —— 检测可用的 GPU 编码器（macOS 上的 VideoToolbox，Windows 上的 NVENC）
4. **自动下载** —— 如果未找到 FFmpeg，则自动下载并解压

## 数据流

### IPC 命令

前端到后端的通信使用 Tauri 的 `invoke()`：

```
Vue Component
  → useAppState composable
    → tauri invoke("command_name", { args })
      → Rust #[command] handler
        → Modifies AppState / performs I/O
          → Returns result to frontend
```

### 事件发送

后端到前端的通信使用 Tauri 事件：

```
Rust backend
  → app_handle.emit("event-name", payload)
    → Frontend event listener (registered in useAppState)
      → Updates reactive state
        → Vue re-renders UI
```

关键事件包括压缩进度更新、扫描完成通知和队列状态变更。

## 跨平台注意事项

| 关注点 | macOS | Windows |
|---------|-------|---------|
| FFmpeg 来源 | Homebrew / 内置 | 内置 / 自动下载 |
| 硬件编码器 | VideoToolbox (`h264_videotoolbox`) | NVENC (`h264_nvenc`) |
| 挂载监控 | 卷挂载检测 | 驱动器可用性检查 |
| 自动启动 | Launch Agent | 注册表 / 启动文件夹 |
| 构建输出 | `.dmg`（通过 `build_dmg.sh`） | `.msi` / `.exe`（通过 Tauri） |
