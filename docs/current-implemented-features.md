# 当前已完成功能梳理

> 说明：本文只基于当前仓库中的实际代码整理，优先统计“已经写出来并能跑通/被调用”的能力，不把架构文档中的规划项算作已完成。

## 1. 当前已经搭好的整体工程

- 已建立 `pnpm workspace + Rust workspace` 的 monorepo 结构。
- 已拆分出桌面设置中心、共享类型、共享 UI、Rust 核心模块、原生宿主模块、SQLite schema、架构文档等目录。
- 已配置常用开发命令：
  - `pnpm dev` / `pnpm dev:desktop`
  - `pnpm dev:service`
  - `pnpm dev:all`
  - `pnpm build`
  - `pnpm typecheck`
  - `pnpm check:rust`
  - `pnpm test:rust`

## 2. 桌面设置中心（Electron + Vue 3）已实现内容

### 2.1 Electron 应用骨架已完成

- 已完成 Electron 主进程窗口创建。
- 已配置开发态加载 `http://localhost:5173`，生产态加载本地打包页面。
- 已启用 `preload`、`contextIsolation`，关闭 `nodeIntegration`。
- 已具备桌面端构建、类型检查和开发热更新能力。

### 2.2 Renderer 与主进程 IPC 调用链已打通

- `preload` 已向渲染进程暴露 `window.imeApi`。
- 当前已实现两个 IPC 接口：
  - `getDashboard()`
  - `getRuntimeStatus()`
- Vue 页面已在 `onMounted` 时异步拉取上述数据。

### 2.3 设置中心首页 UI 已落地

- 已实现一个可运行的 Dashboard 页面。
- 已实现左侧导航壳组件 `AppShell`。
- 已实现指标卡片组件 `MetricCard`。
- 已实现通用信息面板组件 `SectionPanel`。
- 已完成基础视觉样式，包括浅色背景、渐变背景、卡片式布局。

### 2.4 共享前端包已拆分

- `packages/shared-types` 已提供设置中心导航、指标、运行状态等前端共享类型。
- `packages/shared-ui` 已提供可复用 Vue UI 组件。

### 2.5 当前前端能力边界

- Dashboard 当前展示的数据是静态示例数据，不是从 Rust service 或数据库实时读取。
- 左侧导航目前主要是信息架构和展示入口，尚未展开成完整多页面设置功能。
- 还没有真正的“配置修改 -> 持久化 -> 回显”闭环。

## 3. Rust 输入法核心已实现内容

### 3.1 配置模型已完成

- `ime-config` 已定义完整的应用配置结构：
  - `general`
  - `input`
  - `appearance`
  - `logging`
- 已提供默认配置值，例如默认方案 `pinyin`、默认候选页大小、默认主题等。

### 3.2 平台无关数据结构已完成

- `ime-platform-api` 已定义输入法核心与平台宿主之间的通用模型：
  - `InputMode`
  - `Candidate`
  - `CandidatePage`
  - `PreeditState`
  - `PlatformHost` trait

### 3.3 词典接口和内存词典样例已完成

- `ime-dict` 已抽象 `DictionaryProvider`。
- 已实现一个 `MemoryDictionary` 示例词典。
- 当前可根据输入编码返回示例候选词，供核心引擎和原生宿主联调。

### 3.4 输入法状态机与事件处理已完成

- `ime-core` 已实现 `SessionState`，维护：
  - 原始输入
  - 组合串
  - 当前候选
  - 选中索引
  - 中英文模式
- 已实现按键事件模型 `KeyEvent`。
- 已支持的核心输入行为：
  - 字符输入
  - Backspace
  - Enter
  - Space
  - Escape
  - 数字选词
  - 中英文模式切换
- 已实现：
  - preedit 生成
  - 候选刷新
  - 数字选词提交
  - 空格/回车确认首候选
  - 无候选时回退提交原始编码
  - 英文模式下直接上屏
  - 取消输入时清空会话

### 3.5 Rust 核心已有基础测试

- 已覆盖至少以下行为测试：
  - 中文模式下生成 preedit 和候选
  - 数字键选择候选并提交
  - 模式切换后英文直通

## 4. 数据存储与 SQLite 已实现内容

### 4.1 数据库初始化与配置持久化已完成

- `ime-db` 已实现 SQLite 打开、初始化、读取配置、保存配置。
- 初始化时会自动执行 schema。
- 如果数据库中没有应用配置，会自动写入默认配置。
- 已有配置读写 round-trip 测试。

### 4.2 数据表结构已建立

- 当前 SQL schema 已定义以下表：
  - `app_config`
  - `input_schema`
  - `user_dictionary`
  - `input_history`
  - `snippets`
  - `hotkeys`
  - `error_logs`
- 已建立用户词典和输入历史的索引。

### 4.3 当前真正接入的持久化范围

- 目前代码中真正读写打通的是 `app_config`。
- 其他表目前已经建好 schema，但还没有完整业务读写实现。

## 5. Rust Service 已实现内容

- `ime-service` 已能启动并初始化本地数据库。
- 已能读取默认配置。
- 已内置一次最小输入流程 smoke run：
  - 输入字符
  - 触发候选
  - 使用空格提交
- 已支持 `--oneshot` 模式，便于快速验证服务骨架。
- 非 `--oneshot` 模式下可以作为长驻开发服务运行。

### 当前 service 的边界

- 目前更像“开发态骨架服务”，还不是完整的后台输入法服务进程。
- 还没有真正的 IPC 服务端实现、配置广播、日志聚合、宿主联动等完整能力。

## 6. Windows TSF 原生宿主已实现内容

### 6.1 Windows TSF 基础启动已完成

- `windows-tsf` 已作为独立 Rust crate 接入工作区。
- 已实现 COM STA 初始化。
- 已实现 `ITfThreadMgr` 创建和激活。

### 6.2 最小 Text Service Processor 已完成

- 已实现最小 `ITfTextInputProcessor` 对象。
- 已能跟踪 `Activate / Deactivate` 生命周期。
- 已能记录并暴露 `client_id`。

### 6.3 TSF 文档与上下文辅助能力已完成

- 已实现 document manager 创建。
- 已实现焦点 document 的设置和读取。
- 已实现 context 创建。
- 已实现 context push / pop / top 查询辅助方法。

### 6.4 Windows 宿主到 Rust Core 的联动已完成

- `WindowsTsfHost` 已能把宿主侧按键事件转成 `ime-core::KeyEvent`。
- 已能调用共享 Rust 核心引擎处理输入。
- 已能根据 `EngineResponse` 分别触发：
  - 更新 preedit
  - 更新候选
  - 提交文本
  - 清空会话
  - 切换模式指示

### 6.5 Windows TSF 已有测试覆盖

- 已验证基础输入流程。
- 已验证 document manager 创建。
- 已验证 context 创建。
- 在 Windows 条件下还覆盖了焦点管理、context push/top 等测试。

### 当前 Windows TSF 的边界

- 还没有完成真正的 COM 注册。
- 还没有接入完整的 TSF profile 注册与系统加载。
- 还没有把 preedit、候选窗、commit 真实映射到编辑器上下文。
- 现在属于“原生接入脚手架已打通”，还不是可安装、可系统切换的正式输入法。

## 7. 共享协议与工程辅助模块已实现内容

- `ime-ipc` 已定义基础命令模型，例如：
  - `GetConfig`
  - `GetRuntimeStatus`
  - `ReloadDictionary`
- 已定义 `RuntimeStatusResponse` 结构，作为后续 service/桌面端通信基础。
- `ime-logging` 已提供初始化入口，但目前仍是占位级实现。

## 8. 目前可以认定为“已经完成”的阶段成果

从当前代码来看，已经完成的不是“完整跨平台输入法产品”，而是下面这几个关键阶段成果：

1. 完成了整体仓库和模块拆分，项目结构已经稳定。
2. 完成了桌面设置中心的可运行骨架，以及主进程、preload、renderer 之间的基础 IPC 通路。
3. 完成了 Rust 输入法核心的最小闭环：按键输入、候选生成、选词提交、模式切换。
4. 完成了 SQLite 初始化和应用配置持久化。
5. 完成了 Windows TSF 原生宿主的第一层接入骨架，并且已经能驱动共享 Rust Core。

## 9. 目前还不能算“已完成”的部分

下面这些内容在仓库里更多还是预留、占位或设计阶段，建议不要算进“已完成功能”：

- 真实可编辑的设置页功能
- 配置修改后回写数据库并实时生效
- 用户词典管理、输入历史管理、快捷键管理的完整业务逻辑
- 真正可用的 IPC 通信服务
- 真正可用的日志系统
- macOS IMK 实现
- Linux IBus / fcitx5 实现
- Windows 正式 TSF 注册、安装、系统级输入法切换
- 真实候选窗与编辑器上下文联动

## 10. 一句话总结当前项目状态

当前项目已经完成了“跨平台输入法工程骨架 + Rust 核心最小输入闭环 + Electron 设置中心展示骨架 + SQLite 配置持久化 + Windows TSF 原生接入脚手架”，但距离“完整可用的系统输入法产品”还差配置闭环、真实平台集成、业务页面和正式宿主能力。
