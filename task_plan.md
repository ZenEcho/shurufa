# Task Plan

## Goal
基于当前 Shurufa 仓库的真实实现状态，评估距离“Windows 可日常使用输入法”仍缺少的关键功能，并输出继续开发当前项目的工程化方案、目录设计、设置中心 UI 架构、分阶段路线和可直接交给 AI 的实现提示词。

## Phases
- [complete] Phase 1: 盘点当前仓库结构、文档和关键模块入口
- [in_progress] Phase 2: 评估已完成能力与关键缺口，确定优先级和阶段边界
- [pending] Phase 3: 设计设置中心、IPC、配置闭环、数据管理和日志方案
- [pending] Phase 4: 设计 Windows TSF 正式可用化方案与三阶段路线
- [pending] Phase 5: 整理最终中文方案与可执行提示词

## Decisions
- 坚持在当前工程骨架上演进，不推翻重写。
- 优先 Windows 可用性闭环和设置中心真实闭环，跨平台能力后置。

## Errors Encountered
- `rg --files` 在当前环境调用失败并提示 `Access is denied`，改用 PowerShell `Get-ChildItem`/`Get-Content` 继续盘点。
