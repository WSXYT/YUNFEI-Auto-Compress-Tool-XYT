# AI 维护指南

本文档帮助 AI 代理（Copilot、ChatGPT 等）理解、浏览和维护 YUNFEI Auto Compress Tool 代码库。

## 项目概述

YUNFEI Auto Compress Tool 是一个基于 **Tauri 2** 的桌面应用程序，用于自动化视频压缩。它监控文件夹中的新视频文件，按关键词过滤，并使用 FFmpeg 进行压缩。

**主要特点：**
- 跨平台：macOS + Windows
- 单窗口桌面应用
- 响应式前端与 Rust 后端状态同步
- 所有持久化设置通过 Tauri 的 store 插件存储

## 模块职责

### Rust 后端（`src-tauri/src/`）

| 模块 | 文件 | 用途 |
|--------|------|---------|
| 状态 | `state.rs` | 定义 `AppState` 结构体及所有相关枚举。这是整个应用状态的唯一数据源。 |
| 命令 | `commands.rs` | 所有 `#[tauri::command]` 处理器。这些是前端调用的 IPC 入口点。保持精简 —— 将逻辑委托给其他模块。 |
| FFmpeg | `ffmpeg.rs` | FFmpeg 二进制文件管理：检测、路径解析、版本检查、硬件加速查询和自动下载。 |
| 扫描器 | `scanner.rs` | 扫描配置目录中与关键词过滤器匹配的视频文件。支持基于日期的过滤。 |
| 压缩器 | `compressor.rs` | 根据编解码器、比特率、硬件编码和输出模式设置构建 FFmpeg CLI 参数。 |
| 监控器 | `monitor.rs` | 验证配置的输入目录是否存在且可访问（外部驱动器的挂载状态检查）。 |
| 设置 | `settings.rs` | 使用 Tauri 的 store 插件将 `AppState` 序列化/反序列化到持久化 JSON 存储。 |
| 库 | `lib.rs` | 模块声明（`mod` 语句）和 Tauri 应用构建器（包含命令和插件注册）。 |

### Vue 前端（`src/`）

| 文件 | 用途 |
|------|---------|
| `composables/useAppState.ts` | 核心状态组合式函数。所有组件通过它读取状态和调用操作。 |
| `types/index.ts` | 与 Rust 的 `AppState` 及相关类型对应的 TypeScript 接口。 |
| `components/*.vue` | 按功能区块组织的 UI 组件。每个区块对应应用的一个逻辑区域。 |

## 状态管理模式

### 后端状态

```rust
// 状态封装在 Arc<Mutex<>> 中以实现线程安全共享
pub struct AppState { /* fields */ }

// 在命令中通过 Tauri 的托管状态访问：
#[tauri::command]
fn my_command(state: State<'_, Arc<Mutex<AppState>>>) -> Result<...> {
    let mut s = state.lock().unwrap();
    // 读取/修改 s
}
```

### 前端状态

```typescript
// useAppState 提供响应式状态和操作
const { state, updateSetting, startScan } = useAppState();

// 状态在初始化时从 Rust 同步，每次修改后也会同步
// 来自 Rust 的事件推送实时更新（进度、队列变更）
```

### 状态同步流程

1. **应用启动** → `init()` 调用 `get_state` 命令 → 填充响应式 `state`
2. **用户操作** → 组件调用 `updateSetting(key, value)` → 调用 Rust 命令 → Rust 更新 `AppState` 并持久化 → 返回更新后的状态 → 前端同步
3. **后台事件** → Rust 发出事件 → 前端监听器更新响应式状态

## 常见任务

### 添加新设置项

1. 在 `state.rs` 的 `AppState` 中添加带默认值的字段
2. 在 `settings.rs` 中添加序列化/反序列化
3. 在 `commands.rs` 中添加或更新命令处理器（通常是 `update_settings`）
4. 在 `types/index.ts` 的 TypeScript 接口中添加字段
5. 在 `useAppState.ts` 中暴露（如果使用通用的 `updateSetting`，通常无需更改）
6. 在相关组件中添加 UI 控件

**检查清单：**
- [ ] Rust 类型与 TypeScript 类型匹配
- [ ] 默认值合理
- [ ] 设置在应用重启后持久保存
- [ ] 加载时 UI 显示当前值

### 添加新的 UI 区块

1. 使用 `SectionCard` 容器创建 `src/components/MySection.vue`
2. 导入到 `App.vue` 并放置在布局中
3. 使用 `useAppState()` 获取状态和操作
4. 遵循现有区块的模式以保持一致性

### 添加新的 Rust 命令

1. 在 `commands.rs` 中定义：
   ```rust
   #[tauri::command]
   pub fn my_command(state: State<'_, Arc<Mutex<AppState>>>) -> Result<String, String> {
       // 实现
   }
   ```
2. 在 `lib.rs` 的 invoke handler 列表中注册
3. 从前端调用：`await invoke('my_command', { args })`

## 测试注意事项

- **目前没有自动化测试套件。** 验证通过手动方式进行。
- 进行平台特定更改时，需在 macOS 和 Windows 上都进行测试
- 使用系统安装的 FFmpeg 和内置的 FFmpeg 分别验证 FFmpeg 操作
- 使用各种视频格式和文件大小进行测试
- 检查应用重启后设置是否持久保存
- 在有 GPU 和无 GPU 支持的系统上验证硬件编码检测

## 首先需要了解的关键文件

如果您是第一次接触此代码库，请按以下顺序阅读这些文件：

1. `src-tauri/src/state.rs` —— 了解数据模型
2. `src-tauri/src/commands.rs` —— 了解 API 接口
3. `src/composables/useAppState.ts` —— 了解前端状态管理
4. `src/App.vue` —— 了解组件布局
5. `src-tauri/src/lib.rs` —— 了解模块和命令注册
