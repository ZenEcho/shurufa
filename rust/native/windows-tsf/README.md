# windows-tsf

Windows-first native host scaffold for the input method.

## What Exists Today

- A standalone Rust crate inside the workspace
- A `WindowsTsfHost` type that translates host-side key events into `ime-core`
- A real TSF runtime bootstrap that initializes COM STA and activates `ITfThreadMgr`
- A minimal `ITfTextInputProcessor` COM object that tracks activate/deactivate lifecycle
- Document manager creation plus focus set/get helpers through TSF
- Context creation plus push/top helpers through `ITfDocumentMgr`
- Tests proving the host can drive the shared Rust session engine

## What Still Needs To Be Built

- COM registration for the TSF text service
- Full `ITfTextInputProcessor` profile registration and loader integration
- Focus context tracking beyond the current document manager helpers
- Composition lifecycle and edit-session integration on top of the current context helpers
- Candidate UI element updates through TSF APIs
- Commit and preedit mapping to real editor contexts

## Why This Shape

The shared Rust core now owns input state transitions and candidate generation.
The Windows host only needs to:

1. Capture Windows key events and text context callbacks
2. Convert them into `ime-core::KeyEvent`
3. Apply `EngineResponse` back to TSF APIs

That keeps platform-specific code thin and lets the business logic stay shared.
