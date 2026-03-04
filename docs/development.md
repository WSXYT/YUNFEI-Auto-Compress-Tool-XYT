# 开发指南

## 前置条件

| 要求 | 版本 |
|-------------|---------|
| [Node.js](https://nodejs.org/) | 18+ |
| [Rust](https://rustup.rs/) | stable（最新版） |
| npm | 随 Node.js 捆绑 |

### 平台特定依赖

**macOS：**
- Xcode 命令行工具：`xcode-select --install`

**Windows：**
- [Microsoft Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)（Windows 10+ 已预装）

## 设置

```bash
# 克隆仓库
git clone https://github.com/your-org/YUNFEI-Auto-Compress-Tool-XYT.git
cd YUNFEI-Auto-Compress-Tool-XYT

# 安装前端依赖
npm install

# 以开发模式运行（同时启动 Vite 开发服务器和 Tauri）
npm run tauri dev
```

开发服务器会打开一个启用了前端热重载的原生窗口。Rust 代码的更改会触发自动重新构建。

## 构建

```bash
# 构建生产版本
npm run tauri build
```

构建输出位于 `src-tauri/target/release/bundle/`。在 macOS 上，使用 `build_dmg.sh` 创建 `.dmg` 安装包。

## 项目结构

```
├── src/                        # Vue 3 前端
│   ├── components/             # UI 组件（10 个组件）
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
│   │   └── useAppState.ts      # 核心状态管理
│   ├── types/
│   │   └── index.ts            # TypeScript 类型定义
│   ├── App.vue                 # 根组件
│   └── main.ts                 # Vue 应用入口点
├── src-tauri/                  # Rust 后端
│   ├── src/
│   │   ├── main.rs             # Tauri 入口点
│   │   ├── lib.rs              # 模块注册
│   │   ├── commands.rs         # IPC 命令处理器
│   │   ├── compressor.rs       # FFmpeg 参数构建器
│   │   ├── ffmpeg.rs           # FFmpeg 检测和管理
│   │   ├── monitor.rs          # 文件夹挂载监控
│   │   ├── scanner.rs          # 视频文件扫描器
│   │   ├── settings.rs         # 持久化设置
│   │   └── state.rs            # 应用状态
│   ├── Cargo.toml              # Rust 依赖
│   └── tauri.conf.json         # Tauri 配置
├── docs/                       # 文档
├── index.html                  # HTML 入口点
├── package.json                # Node.js 依赖和脚本
├── vite.config.ts              # Vite 配置
├── tsconfig.json               # TypeScript 配置
└── build_dmg.sh                # macOS DMG 构建脚本
```

## 添加功能

### 添加新设置项

1. **状态** —— 在 `src-tauri/src/state.rs` 的 `AppState` 中添加字段
2. **设置** —— 在 `src-tauri/src/settings.rs` 中处理序列化
3. **命令** —— 在 `src-tauri/src/commands.rs` 中添加或更新命令
4. **类型** —— 在 `src/types/index.ts` 的 TypeScript 接口中添加字段
5. **组合式函数** —— 在 `src/composables/useAppState.ts` 中暴露该设置
6. **组件** —— 在相应的 `src/components/*.vue` 文件中添加 UI 控件

### 添加新的 UI 区块

1. 在 `src/components/` 中创建新组件（使用 `SectionCard` 作为容器）
2. 在 `src/App.vue` 中导入并放置组件
3. 连接 `useAppState` 以获取状态和操作

### 添加新的 Rust 命令

1. 在 `src-tauri/src/commands.rs` 中使用 `#[tauri::command]` 定义函数
2. 在 `src-tauri/src/lib.rs` 的 `invoke_handler` 中注册
3. 从前端使用 `invoke("command_name", { args })` 调用

## 调试

### 前端

- 在 Tauri 窗口中打开开发者工具：右键点击 → "检查元素"（仅限调试版本）
- Vue DevTools 浏览器扩展可与 Tauri 的 WebView 配合使用
- 检查浏览器控制台中的 IPC 错误

### 后端

- Rust 日志会输出到运行 `npm run tauri dev` 的终端
- 使用 `println!()` 或 `log` crate 进行调试输出
- 设置 `RUST_LOG=debug` 环境变量以获取详细日志

### 常见问题

| 问题 | 解决方案 |
|-------|----------|
| 找不到 FFmpeg | 使用 RuntimeSection 中的"刷新"按钮，或手动设置路径 |
| macOS 上构建失败 | 确保已安装 Xcode 命令行工具：`xcode-select --install` |
| Windows 上构建失败 | 安装 Visual Studio C++ Build Tools |
| 热重载不工作 | 重启 `npm run tauri dev` —— Vite HMR 偶尔会断开连接 |
