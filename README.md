# localdrop

一个点对点的局域网文件传输工具，支持大文件传输、断点续传、多平台支持。

## 功能特性

- ✅ **大文件传输** - 支持任意大小的文件传输
- ✅ **断点续传** - 传输中断后可从断点继续
- ✅ **多线程传输** - 提高传输速度
- ✅ **进度和速度显示** - 实时显示传输进度和速度
- ✅ **取消传输** - 支持取消发送，接收方自动清理已缓存的文件块
- ✅ **设备重命名** - 自定义设备名称
- ✅ **多文件/文件夹传输** - 支持批量传输
- ✅ **跨平台** - Windows、macOS、Android、iOS

## 技术栈

### 前端
- React 18 + TypeScript
- Vite 7
- Tailwind CSS 3
- Lucide React (图标)
- Zustand (状态管理)
- React Router DOM

### 后端
- Tauri 2.0
- Rust
- Tokio (异步运行时)
- SQLite (持久化存储)
- TCP/UDP (网络通信)

## 快速开始

### 前置要求

- Node.js 20+
- Rust stable
- Tauri CLI

### 安装依赖

```bash
npm install
```

### 开发模式

```bash
npm run tauri dev
```

### 构建发布

```bash
npm run tauri build
```

## 开发指南

### 项目结构

```
.
├── src/                    # 前端源代码
│   ├── components/         # React 组件
│   ├── pages/              # 页面组件
│   ├── stores/             # Zustand 状态管理
│   ├── types/              # TypeScript 类型定义
│   └── App.tsx             # 主应用组件
├── src-tauri/              # Tauri 后端代码
│   ├── src/
│   │   ├── commands.rs     # Tauri 命令
│   │   ├── device/         # 设备发现模块
│   │   ├── transfer/       # 文件传输模块
│   │   ├── file/           # 文件处理模块
│   │   ├── protocol/       # 协议定义
│   │   └── persistence/    # 持久化存储
│   └── Cargo.toml          # Rust 依赖
├── .github/workflows/      # GitHub Actions
├── package.json            # 前端依赖
└── tauri.conf.json         # Tauri 配置
```

### 主要组件

| 组件 | 说明 |
|------|------|
| DeviceCard | 设备卡片组件 |
| ProgressBar | 进度条组件 |
| HomePage | 首页（设备列表） |
| TransferPage | 传输列表页 |
| SettingsPage | 设置页 |

## 构建指南

### Windows

```bash
npm run tauri build -- --target x86_64-pc-windows-msvc
```

产物包括：
- MSI 安装包
- NSIS 安装包
- Portable ZIP（免安装版本）

### macOS

```bash
npm run tauri build -- --target aarch64-apple-darwin
```

产物：
- DMG 镜像

### Android

```bash
npm run tauri android init
npm run tauri android build
```

产物：
- APK
- AAB

### iOS

```bash
npm run tauri ios init
npm run tauri ios build
```

产物：
- IPA

## GitHub Actions

项目配置了自动构建 workflow，推送 `v*` 标签时自动构建并上传到 Releases：

```bash
git tag v0.1.0
git push origin v0.1.0
```

## 许可证

MIT