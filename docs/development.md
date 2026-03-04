# Development Guide

## Prerequisites

| Requirement | Version |
|-------------|---------|
| [Node.js](https://nodejs.org/) | 18+ |
| [Rust](https://rustup.rs/) | stable (latest) |
| npm | Bundled with Node.js |

### Platform-Specific Dependencies

**macOS:**
- Xcode Command Line Tools: `xcode-select --install`

**Windows:**
- [Microsoft Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (pre-installed on Windows 10+)

## Setup

```bash
# Clone the repository
git clone https://github.com/your-org/YUNFEI-Auto-Compress-Tool-XYT.git
cd YUNFEI-Auto-Compress-Tool-XYT

# Install frontend dependencies
npm install

# Run in development mode (starts both Vite dev server and Tauri)
npm run tauri dev
```

The development server opens a native window with hot-reload enabled for the frontend. Rust changes trigger an automatic rebuild.

## Building

```bash
# Build for production
npm run tauri build
```

Build output is located in `src-tauri/target/release/bundle/`. On macOS, use `build_dmg.sh` to create a `.dmg` installer.

## Project Structure

```
├── src/                        # Vue 3 frontend
│   ├── components/             # UI components (10 components)
│   │   ├── HeaderSection.vue
│   │   ├── MonitorSection.vue
│   │   ├── ScanSection.vue
│   │   ├── CompressSection.vue
│   │   ├── OutputSection.vue
│   │   ├── RuntimeSection.vue
│   │   ├── LogSection.vue
│   │   ├── SectionCard.vue
│   │   ├── HelpDialog.vue
│   │   └── DonateDialog.vue
│   ├── composables/
│   │   └── useAppState.ts      # Central state management
│   ├── types/
│   │   └── index.ts            # TypeScript type definitions
│   ├── App.vue                 # Root component
│   └── main.ts                 # Vue app entry point
├── src-tauri/                  # Rust backend
│   ├── src/
│   │   ├── main.rs             # Tauri entry point
│   │   ├── lib.rs              # Module registration
│   │   ├── commands.rs         # IPC command handlers
│   │   ├── compressor.rs       # FFmpeg argument builder
│   │   ├── ffmpeg.rs           # FFmpeg detection & management
│   │   ├── monitor.rs          # Folder mount monitoring
│   │   ├── scanner.rs          # Video file scanner
│   │   ├── settings.rs         # Persistent settings
│   │   └── state.rs            # Application state
│   ├── Cargo.toml              # Rust dependencies
│   └── tauri.conf.json         # Tauri configuration
├── docs/                       # Documentation
├── index.html                  # HTML entry point
├── package.json                # Node.js dependencies & scripts
├── vite.config.ts              # Vite configuration
├── tsconfig.json               # TypeScript configuration
└── build_dmg.sh                # macOS DMG builder script
```

## Adding Features

### Adding a New Setting

1. **State** — Add the field to `AppState` in `src-tauri/src/state.rs`
2. **Settings** — Handle serialization in `src-tauri/src/settings.rs`
3. **Commands** — Add or update a command in `src-tauri/src/commands.rs`
4. **Types** — Add the field to the TypeScript interface in `src/types/index.ts`
5. **Composable** — Expose the setting in `src/composables/useAppState.ts`
6. **Component** — Add UI controls in the appropriate `src/components/*.vue` file

### Adding a New UI Section

1. Create a new component in `src/components/` (use `SectionCard` as wrapper)
2. Import and place it in `src/App.vue`
3. Connect it to `useAppState` for state and actions

### Adding a New Rust Command

1. Define the function with `#[tauri::command]` in `src-tauri/src/commands.rs`
2. Register it in the `invoke_handler` in `src-tauri/src/lib.rs`
3. Call it from the frontend using `invoke("command_name", { args })`

## Debugging

### Frontend

- Open DevTools in the Tauri window: right-click → "Inspect Element" (debug builds only)
- Vue DevTools browser extension works with Tauri's WebView
- Check the browser console for IPC errors

### Backend

- Rust logs are printed to the terminal where `npm run tauri dev` is running
- Use `println!()` or the `log` crate for debug output
- Set `RUST_LOG=debug` environment variable for verbose logging

### Common Issues

| Issue | Solution |
|-------|----------|
| FFmpeg not found | Use the "Refresh" button in RuntimeSection, or manually set the path |
| Build fails on macOS | Ensure Xcode CLI tools are installed: `xcode-select --install` |
| Build fails on Windows | Install Visual Studio C++ Build Tools |
| Hot reload not working | Restart `npm run tauri dev` — Vite HMR occasionally disconnects |
