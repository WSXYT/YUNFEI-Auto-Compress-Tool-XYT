# Architecture

## System Overview

YUNFEI Auto Compress Tool is built on **Tauri 2**, which combines a Rust backend with a web-based frontend. The architecture follows a clear separation of concerns:

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

- **Frontend** renders the UI and captures user interactions.
- **Tauri IPC** bridges frontend calls to Rust command handlers.
- **Backend** manages state, file scanning, video compression, and system integration.

## Frontend Architecture

### Entry Point

`src/main.ts` creates the Vue app and mounts it to `#app` in `index.html`.

### Components

All components live in `src/components/`:

| Component | Responsibility |
|-----------|---------------|
| `HeaderSection.vue` | App title and author info |
| `MonitorSection.vue` | Input folder management (primary + 2 extra folders) |
| `ScanSection.vue` | Keyword filtering (up to 3 keywords) |
| `CompressSection.vue` | Codec and quality preset selection (H.264/H.265) |
| `OutputSection.vue` | Output mode configuration (overwrite, folder, suffix) |
| `RuntimeSection.vue` | FFmpeg management, queue status, compression progress |
| `LogSection.vue` | Recent logs and update history |
| `SectionCard.vue` | Reusable card wrapper for UI sections |
| `HelpDialog.vue` | Help modal |
| `DonateDialog.vue` | Donation modal |

### Composables

`src/composables/useAppState.ts` is the central state management composable. It provides:

- Reactive application state synced with the Rust backend
- Methods: `init()`, `updateSetting()`, `startScan()`, `stopScan()`, `pickFolder()`, `pickFfmpeg()`, `refreshFfmpeg()`, `clearQueue()`, and more
- Event listeners for backend-emitted events (progress updates, scan results)

### Types

`src/types/index.ts` defines TypeScript interfaces for the shared state model, mirroring the Rust `AppState` struct.

## Backend Architecture

### Module Overview

All Rust source files are in `src-tauri/src/`:

| Module | File | Responsibility |
|--------|------|---------------|
| **State** | `state.rs` | Core `AppState` struct and related enums (`CodecOption`, `OutputMode`, `TimeGateOption`, etc.) |
| **Commands** | `commands.rs` | Tauri `#[command]` handlers — the IPC entry points called from the frontend |
| **FFmpeg** | `ffmpeg.rs` | FFmpeg binary detection, path resolution, hardware acceleration checks, auto-download |
| **Scanner** | `scanner.rs` | Scans directories for video files matching keywords; filters by date |
| **Compressor** | `compressor.rs` | Builds FFmpeg arguments for compression (codec, bitrate, audio settings) |
| **Monitor** | `monitor.rs` | Validates input directory availability; checks folder mount status |
| **Settings** | `settings.rs` | Loads/saves app configuration to persistent JSON storage |
| **Lib** | `lib.rs` | Module declarations and Tauri plugin/command registration |

### State Management

`AppState` is the single source of truth, wrapped in `Arc<Mutex<>>` and shared across all command handlers. Key fields include:

- Input paths (primary + extra folders)
- Keywords and scan settings
- Codec, bitrate, and output mode configuration
- FFmpeg path and hardware acceleration status
- Compression queue and progress

### FFmpeg Integration

The `ffmpeg` module handles:

1. **Detection** — Searches system PATH and common install locations
2. **Validation** — Verifies the binary works and checks version
3. **Hardware acceleration** — Detects available GPU encoders (VideoToolbox on macOS, NVENC on Windows)
4. **Auto-download** — Downloads and extracts FFmpeg if not found

## Data Flow

### IPC Commands

Frontend-to-backend communication uses Tauri's `invoke()`:

```
Vue Component
  → useAppState composable
    → tauri invoke("command_name", { args })
      → Rust #[command] handler
        → Modifies AppState / performs I/O
          → Returns result to frontend
```

### Event Emission

Backend-to-frontend communication uses Tauri events:

```
Rust backend
  → app_handle.emit("event-name", payload)
    → Frontend event listener (registered in useAppState)
      → Updates reactive state
        → Vue re-renders UI
```

Key events include compression progress updates, scan completion notifications, and queue status changes.

## Cross-Platform Considerations

| Concern | macOS | Windows |
|---------|-------|---------|
| FFmpeg source | Homebrew / bundled | Bundled / auto-download |
| Hardware encoder | VideoToolbox (`h264_videotoolbox`) | NVENC (`h264_nvenc`) |
| Mount monitoring | Volume mount detection | Drive availability checks |
| Auto-start | Launch Agent | Registry / Startup folder |
| Build output | `.dmg` via `build_dmg.sh` | `.msi` / `.exe` via Tauri |
