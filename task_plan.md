# Task Plan

## 目标
基于当前 `D:\codex\private\shurufa` 仓库继续推进 Windows 优先的桌面输入法项目，优先补齐：
- 设置中心真实闭环
- 输入测试 MVP
- Windows TSF 正式化骨架

## 已完成阶段
- [x] 阶段 1：盘点仓库结构、文档和核心模块入口
- [x] 阶段 2：完成设置中心真实配置闭环
- [x] 阶段 3：完成用户词典 / 历史 / 热键 / 日志页最小数据闭环
- [x] 阶段 4：完成 Windows TSF 第一批正式化骨架
- [x] 阶段 5：完成项目内“可真实打字”的输入测试 MVP

## 当前阶段
- [in_progress] 阶段 6：推进 Windows TSF 到可注册 DLL / 可安装骨架

## 当前切片
- [x] `windows-tsf` 新增 `response.rs`、`composition.rs`、`context_bridge.rs`
- [x] `WindowsTsfHost` 已打通 `EngineResponse -> HostAction -> CompositionAction -> TextWriteRequest`
- [x] `windows-tsf` 已新增 `registration.rs`，包含 CLSID、profile GUID、TSF 注册骨架
- [x] `windows-tsf` 已支持 `cdylib` 输出，release 构建可生成 `rust\target\release\windows_tsf.dll`
- [x] 已导出：
  - `DllRegisterServer`
  - `DllUnregisterServer`
  - `DllCanUnloadNow`
  - `DllGetClassObject`
- [x] 已更新：
  - `rust\native\windows-tsf\scripts\register-ime.ps1`
  - `rust\native\windows-tsf\scripts\unregister-ime.ps1`
- [ ] 待完成：真实 COM 类工厂
- [ ] 待完成：真实 TSF edit session / composition 写回
- [ ] 待完成：系统输入法列表可见与可切换实机闭环

## 下一步
1. 为 `DllGetClassObject` 补最小类工厂骨架。
2. 把 `TextWriteRequest` 映射到真实 TSF 编辑上下文。
3. 做最小候选窗和焦点联动。
4. 完成“记事本可直接打字”的 Windows 端闭环。
