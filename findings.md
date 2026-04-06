# Findings

## Repository Snapshot
- 根目录包含 `apps/`、`packages/`、`rust/`、`docs/`，并已有 `shurufa.db`。
- 仓库当前不是 Git working tree，`git status` 和 `git log` 不可用。
- README 明确指出：Electron 是设置中心；`windows-tsf` 仍是 scaffold，尚未注册为真实系统 IME。

## Docs
- `docs/ime-architecture.md`
- `docs/current-implemented-features.md`

## Verified Code State
- `apps/desktop-settings` 当前仅实现 `ime:get-dashboard`、`ime:get-runtime-status` 两个 IPC 接口，返回共享包里的静态示例数据。
- 桌面端还未接入 Naive UI，当前 UI 主要是自定义 Vue SFC + UnoCSS。
- `ime-core` 已完成字符输入、Backspace、Enter、Space、Escape、数字选词、中英文切换、preedit 和候选返回，以及无候选时提交原始编码。
- `ime-db` 已完成 SQLite schema 初始化和 `app_config` 的读写闭环，但其余业务表尚未有完整 CRUD。
- `ime-ipc` 只定义了极少量命令枚举，尚未形成真正的 IPC 服务协议层。
- `ime-logging` 目前仍是占位实现，只打印一条初始化日志。
- `windows-tsf` 已完成 COM STA、`ITfThreadMgr`、最小 `ITfTextInputProcessor` 生命周期、document/context helper，并能把按键送入 Rust Core；但 `PlatformHost` 侧仍只有日志输出，没有真实编辑上下文映射。
- `rust/native/macos-imk`、`rust/native/linux-ibus`、`rust/native/linux-fcitx5` 目前只有 README，没有实装代码。

## Verification
- `pnpm typecheck` 通过。
- `cargo test --manifest-path rust/Cargo.toml` 通过，包含 `ime-core`、`ime-db`、`windows-tsf` 的单元测试。
