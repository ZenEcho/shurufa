# Shurufa

Cross-platform PC input method scaffold for Windows, macOS, and Linux.

## Stack

- Desktop settings center: Electron + Vue 3 + TypeScript + UnoCSS
- Input method core: Rust
- Local storage: SQLite

## Repository Layout

```text
apps/                 Electron settings center
packages/             Shared frontend types and UI components
rust/                 Rust workspace, core crates, and native host scaffolds
docs/                 Architecture and implementation notes
```

## Current Status

- Electron settings center scaffold builds successfully.
- Rust workspace compiles and tests successfully.
- `ime-core` now supports key events, candidate selection, commit flow, and mode toggle.
- `ime-db` now initializes SQLite schema and persists app config.
- `windows-tsf` now includes COM bootstrap, `ITfThreadMgr` activation, a minimal `ITfTextInputProcessor`, `ITfDocumentMgr` focus management, and TSF context creation/push/top helpers wired to the Rust core.

## Setup

```bash
pnpm install
```

## What To Start

### 1. Desktop settings center

This is the main frontend app in the repository today.

```bash
pnpm dev
```

Or explicitly:

```bash
pnpm dev:desktop
```

### 2. Rust service

Start this when you want to verify the Rust backend scaffold, SQLite initialization, and core engine flow.

```bash
pnpm dev:service
```

This command is now long-running and behaves like a local dev service.

If you only want a quick smoke run that exits immediately:

```bash
pnpm dev:service:oneshot
```

### 3. Start both together

```bash
pnpm dev:all
```

## Startup Recommendation

### Daily frontend development

Start only:

```bash
pnpm dev
```

That is enough for:

- Electron window
- Vue settings UI
- renderer/main/preload hot reload

### Full local development

Open two terminals.

Terminal 1:

```bash
pnpm dev
```

Terminal 2:

```bash
pnpm dev:service
```

Use this mode when you want to work on both:

- Electron settings center
- Rust service / SQLite / core engine

You can also use one command instead:

```bash
pnpm dev:all
```

### What does not need to be started yet

You do **not** need to start a real Windows TSF text service process yet.
The `windows-tsf` crate is currently a native integration scaffold inside the Rust workspace, not a registered system IME installer.

## pnpm Note

This repo uses `pnpm` workspace mode and allows build scripts for `electron` and `esbuild`, so `pnpm install` is the correct setup step before the first `pnpm dev`.

## Verification Commands

### Frontend

```bash
pnpm typecheck
pnpm build
```

### Rust

```bash
pnpm check:rust
pnpm test:rust
```

## Notes

- The Electron app is the settings and diagnostics shell, not the system IME host.
- The Windows TSF integration is still a scaffold and does not yet register a real COM text service.
- The detailed architecture document is at [docs/ime-architecture.md](D:\codex\private\shurufa\docs\ime-architecture.md).
