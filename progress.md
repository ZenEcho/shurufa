# Progress

## 2026-04-06

### 设置中心闭环
- 已把 Electron 设置中心接到真实 Rust 服务，不再依赖静态示例数据。
- 已接入 Naive UI 与 Pinia，并拆出 `AppShell`、总览页、输入设置页等页面结构。
- 已完成配置读取、保存、重置、回显。

### 数据管理页面
- 已补齐用户词典、输入历史、热键、日志四个页面的最小真实数据链路。
- 已补齐 Electron main / preload / renderer 到 Rust service 的对应接口。
- 已修复 Rust `snake_case` 和前端 `camelCase` 的字段映射问题。

### 输入测试 MVP
- 已新增项目内“输入测试”页，可直接手动打字验证 Rust 输入核心。
- 已新增：
  - `create_typing_session`
  - `process_typing_key`
  - `run-typing-demo`
- 已新增脚本：
  - `scripts\test-typing-mvp.ps1`
  - `scripts\verify-mvp.ps1`

### Windows TSF 正式化骨架
- 已完成 `response.rs`、`composition.rs`、`context_bridge.rs`。
- 已完成 TSF 注册 manifest 与 profile/category 注册骨架。
- 已让 `WindowsTsfHost` 维护 composition 状态与上下文写请求。

### 本轮新增
- `windows-tsf` 已新增 `cdylib` 输出类型。
- release 构建已生成 `rust\target\release\windows_tsf.dll`。
- 已导出：
  - `DllRegisterServer`
  - `DllUnregisterServer`
  - `DllCanUnloadNow`
  - `DllGetClassObject`
- 注册脚本已改为围绕 release DLL 和 `regsvr32` 的最小调用链。
- TSF 默认显示名已改为“书入法输入法”。

### 最新验证
- `cargo test --manifest-path rust\Cargo.toml -p ime-core -p ime-service -p windows-tsf`
- `cargo test --manifest-path rust\Cargo.toml -p windows-tsf registration`
- `cargo build --manifest-path rust\Cargo.toml -p windows-tsf --release`
- `pnpm typecheck`
- `pnpm build`
- `pnpm test:typing`
- `Test-Path rust\target\release\windows_tsf.dll`

### 下一步
- 补真实 COM 类工厂。
- 把 `TextWriteRequest` 映射到真实 TSF 编辑上下文。
- 做最小候选窗、焦点处理和系统输入法实机链路。
