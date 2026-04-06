# Findings

## 仓库现状
- 仓库不是空白项目，已经具备 Electron 设置中心、Rust 输入核心、SQLite 初始化和 Windows TSF 宿主骨架。
- 前端栈固定为 Electron + Vue 3 + TypeScript + Naive UI + UnoCSS，当前已实际接入。
- 后端以 Rust workspace 组织，核心目录在 `rust\crates` 和 `rust\native\windows-tsf`。

## 已验证能力
- 设置中心已不再使用静态示例数据，配置读写真实走 Rust 服务和 SQLite。
- 用户词典、输入历史、热键、日志页面已具备最小真实数据读写能力。
- 项目内已新增“输入测试”页，可直接键盘输入并驱动 Rust `ime-core`：
  - 字母输入
  - Backspace
  - Enter
  - Space
  - Escape
  - 数字选词
  - 中英文切换
- `windows-tsf` 当前已具备：
  - TSF 注册 manifest
  - composition 生命周期状态
  - 文本上下文桥接请求队列
  - DLL 自注册导出骨架

## 当前边界
- 还没有真正可安装并可系统切换的 Windows 输入法。
- `DllGetClassObject` 目前仍是占位返回，尚未提供真实类工厂。
- `TextWriteRequest` 还没有真正写入 `ITfContext`。
- 候选窗、光标跟随、焦点变化处理还没接到真实 TSF 编辑上下文。

## 本轮新增事实
- `windows-tsf` 已声明 `cdylib`，release 构建可生成 `rust\target\release\windows_tsf.dll`。
- 已新增 DLL 导出：
  - `DllRegisterServer`
  - `DllUnregisterServer`
  - `DllCanUnloadNow`
  - `DllGetClassObject`
- 默认 TSF 显示名已改为中文“书入法输入法”。
- 注册脚本已切到 release DLL + `regsvr32` 的最小链路。

## 仍需优先推进
1. 真实 COM 类工厂。
2. 真实 TSF edit session 写回。
3. 候选窗与 caret 联动。
4. 系统输入法安装、启用、切换和实机验证。
