# AI Maintenance Guide

This document helps AI agents (Copilot, ChatGPT, etc.) understand, navigate, and maintain the YUNFEI Auto Compress Tool codebase.

## Project Overview

YUNFEI Auto Compress Tool is a **Tauri 2** desktop application for automated video compression. It monitors folders for new video files, filters them by keywords, and compresses them using FFmpeg.

**Key characteristics:**
- Cross-platform: macOS + Windows
- Single-window desktop app
- Reactive frontend synced with Rust backend state
- All persistent settings stored via Tauri's store plugin

## Module Responsibilities

### Rust Backend (`src-tauri/src/`)

| Module | File | Purpose |
|--------|------|---------|
| State | `state.rs` | Defines `AppState` struct and all related enums. This is the single source of truth for the entire application state. |
| Commands | `commands.rs` | All `#[tauri::command]` handlers. These are the IPC entry points invoked by the frontend. Keep them thin — delegate to other modules. |
| FFmpeg | `ffmpeg.rs` | FFmpeg binary management: detection, path resolution, version checking, hardware acceleration queries, and auto-download. |
| Scanner | `scanner.rs` | Scans configured directories for video files matching keyword filters. Supports date-based filtering. |
| Compressor | `compressor.rs` | Builds FFmpeg CLI arguments based on codec, bitrate, hardware encoding, and output mode settings. |
| Monitor | `monitor.rs` | Validates that configured input directories exist and are accessible (mount status checks for external drives). |
| Settings | `settings.rs` | Serializes/deserializes `AppState` to/from persistent JSON storage using Tauri's store plugin. |
| Lib | `lib.rs` | Module declarations (`mod` statements) and Tauri app builder with command and plugin registration. |

### Vue Frontend (`src/`)

| File | Purpose |
|------|---------|
| `composables/useAppState.ts` | Central state composable. All components read state and call actions through this. |
| `types/index.ts` | TypeScript interfaces mirroring Rust's `AppState` and related types. |
| `components/*.vue` | UI components organized by feature section. Each section maps to a logical area of the app. |

## State Management Patterns

### Backend State

```rust
// State is wrapped in Arc<Mutex<>> for thread-safe sharing
pub struct AppState { /* fields */ }

// Accessed in commands via Tauri's managed state:
#[tauri::command]
fn my_command(state: State<'_, Arc<Mutex<AppState>>>) -> Result<...> {
    let mut s = state.lock().unwrap();
    // read/modify s
}
```

### Frontend State

```typescript
// useAppState provides reactive state + actions
const { state, updateSetting, startScan } = useAppState();

// State is synced from Rust on init and after each mutation
// Events from Rust push real-time updates (progress, queue changes)
```

### State Sync Flow

1. **App start** → `init()` calls `get_state` command → populates reactive `state`
2. **User action** → component calls `updateSetting(key, value)` → invokes Rust command → Rust updates `AppState` + persists → returns updated state → frontend syncs
3. **Background events** → Rust emits events → frontend listeners update reactive state

## Common Tasks

### Adding a New Setting

1. Add field to `AppState` in `state.rs` with a default value
2. Add serialization/deserialization in `settings.rs`
3. Add or update command handler in `commands.rs` (usually `update_settings`)
4. Add the field to the TypeScript interface in `types/index.ts`
5. Expose in `useAppState.ts` (usually no change needed if using generic `updateSetting`)
6. Add UI control in the relevant component

**Checklist:**
- [ ] Rust type matches TypeScript type
- [ ] Default value is sensible
- [ ] Setting persists across app restarts
- [ ] UI reflects the current value on load

### Adding a New UI Section

1. Create `src/components/MySection.vue` using `SectionCard` wrapper
2. Import into `App.vue` and place in the layout
3. Use `useAppState()` for state and actions
4. Follow existing section patterns for consistency

### Adding a New Rust Command

1. Define in `commands.rs`:
   ```rust
   #[tauri::command]
   pub fn my_command(state: State<'_, Arc<Mutex<AppState>>>) -> Result<String, String> {
       // implementation
   }
   ```
2. Register in `lib.rs` invoke handler list
3. Call from frontend: `await invoke('my_command', { args })`

## Testing Considerations

- **No automated test suite exists currently.** Verification is manual.
- Test on both macOS and Windows when making platform-specific changes
- Verify FFmpeg operations with both system-installed and bundled FFmpeg
- Test with various video formats and file sizes
- Check that settings persist after app restart
- Verify hardware encoding detection on systems with and without GPU support

## Key Files to Understand First

If you are new to this codebase, read these files in order:

1. `src-tauri/src/state.rs` — Understand the data model
2. `src-tauri/src/commands.rs` — Understand the API surface
3. `src/composables/useAppState.ts` — Understand frontend state management
4. `src/App.vue` — Understand the component layout
5. `src-tauri/src/lib.rs` — Understand module and command registration
