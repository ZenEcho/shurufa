# Windows Pinyin MVP Design

## Goal

Build a minimum usable Windows Chinese input method MVP for this repository.
The MVP must register as a Windows TSF input method, switch into an active app, accept a small built-in set of pinyin codes, and commit Chinese text into a real editor such as Notepad.

This is intentionally narrower than the long-term architecture. The purpose of this MVP is to turn the current scaffold into a real end-to-end input method with the smallest possible scope.

## In Scope

- Windows only
- TSF-based input method host
- Pinyin input only
- Small built-in dictionary shipped inside Rust code
- Real preedit and commit flow through the existing TSF host
- Candidate selection with `1..9`
- `Space` and `Enter` commit the first candidate
- `Backspace` deletes the last pending input character
- `Escape` clears the current composition
- Development install and uninstall scripts for local registration
- Automated tests for dictionary lookup and engine behavior
- A manual verification flow in Notepad or VS Code on Windows

## Out of Scope

- macOS or Linux support
- Full pinyin coverage
- Wubi or other schemas
- User dictionary learning
- Candidate paging
- Settings-center integration into the live typing path
- Fancy candidate UI work beyond what is needed for the current TSF scaffold
- Packaging, installer UX, code signing, or production deployment

## Success Criteria

The MVP is successful when all of the following are true:

1. Running the Windows TSF development install script registers the input method in the current user profile.
2. The input method appears in Windows language/input settings.
3. Switching to the input method in Notepad allows typing pinyin and committing Chinese text.
4. The following examples work:
   - `ni` -> `你`
   - `hao` -> `好`
   - `nihao` -> `你好`
   - `zhong` -> `中`
   - `guo` -> `国`
   - `zhongguo` -> `中国`
   - `wo` -> `我`
   - `women` -> `我们`
5. Pressing `Space` or `Enter` with an active composition commits the first candidate.
6. Pressing `1` selects the first candidate and commits it.
7. Pressing `Backspace` updates the preedit correctly.
8. Pressing `Escape` clears the composition without committing text.

## Approach

Use the existing Rust engine and Windows TSF scaffold as the production path for the MVP.
Do not route live typing through Electron or the background service.
The only new behavior in the core path is replacing the sample dictionary with a real built-in pinyin mapping and ensuring the TSF host can commit that output into the focused text context.

This keeps the implementation narrow and aligns with the repository's stated architecture: the desktop app remains a settings shell, while the native TSF host owns system input.

## Architecture

### 1. Dictionary Layer

Replace the current sample `MemoryDictionary` behavior with a deterministic built-in pinyin dictionary.

Design:

- Keep the existing `DictionaryProvider` trait.
- Introduce a built-in static mapping from pinyin code to one or more Chinese candidates.
- Return candidates ordered by fixed priority.
- Candidate IDs remain deterministic, derived from code plus index.

The first version can live inside `ime-dict` as a static table or slice-backed lookup. No database access is required for the MVP.

### 2. Engine Layer

Keep `ime-core` responsible for session state transitions.

Expected engine flow:

- ASCII letters append to `raw_keys`
- `raw_keys` becomes the preedit string
- dictionary lookup returns candidates for the exact code
- `1..9` commits the corresponding candidate
- `Space` or `Enter` commits the first candidate
- if there are no candidates, `Space` or `Enter` commits the raw input as fallback only if we decide to preserve current behavior

For the MVP, the recommended behavior is:

- Preserve fallback commit for unknown codes to avoid trapping the user in a dead composition.

### 3. Windows TSF Host

Use the existing TSF host as the only system integration path.

Responsibilities:

- Receive key events from Windows TSF
- Translate them into `ime-core::KeyEvent`
- Apply engine responses back to the focused TSF text context
- Maintain composition lifecycle state
- Register and unregister the COM text service for the current user

No Electron participation is required in the typing loop.

### 4. Development Registration Flow

Keep the current PowerShell development scripts as the installation surface for the MVP.

Required behavior:

- Build `windows_tsf.dll`
- Register the COM server and TSF language profile under `HKCU`
- Allow uninstall/cleanup for repeated local testing

The scripts should leave the developer with a predictable setup path:

1. Build
2. Register
3. Switch to the IME in Windows
4. Test in Notepad

## Data Flow

The end-to-end runtime flow is:

1. User switches Windows input method to Shurufa.
2. User types a letter in Notepad.
3. TSF key sink receives the keystroke.
4. `windows-tsf` translates the key into `KeyEvent`.
5. `ime-core` updates `SessionState`.
6. `ime-dict` returns candidates for the current pinyin code.
7. `windows-tsf` maps the response into preedit and commit operations for the focused TSF context.
8. On candidate selection or first-candidate commit, Chinese text is inserted into the editor.

## Candidate Set

The built-in dictionary for the MVP should include at least:

- `ni` -> `你`
- `hao` -> `好`
- `nihao` -> `你好`
- `zhong` -> `中`
- `guo` -> `国`
- `zhongguo` -> `中国`
- `wo` -> `我`
- `women` -> `我们`

It is acceptable to include a few additional common words if that simplifies test coverage, but the list should stay intentionally small.

## Error Handling

The MVP should fail simply and visibly:

- If the TSF DLL is not built, install scripts must stop with a clear error.
- If registration fails, the script must exit non-zero.
- If dictionary lookup returns no results, the engine must remain stable and not crash.
- If TSF text execution fails for a context, logging is sufficient for the MVP; no advanced recovery is required.

## Testing Strategy

### Automated

Add or update tests in these layers:

- `ime-dict`: exact-code lookup returns expected Chinese candidates
- `ime-core`: pinyin input builds preedit and commits correct Chinese output
- `windows-tsf`: bound-context tests validate that commit requests contain Chinese text for known codes

Tests should cover:

- exact match lookup
- first-candidate commit through `Space`
- indexed candidate commit through number keys
- backspace behavior
- escape behavior
- unknown-code fallback stability

### Manual

Run a Windows manual verification checklist:

1. Install the development TSF DLL.
2. Confirm registry/profile presence.
3. Switch to the IME in Windows settings or the language switcher.
4. Open Notepad.
5. Verify the sample codes commit the expected Chinese text.
6. Verify `Backspace`, `Space`, `Enter`, and `Escape`.

## Implementation Plan Shape

The implementation should be done in this order:

1. Add failing dictionary tests for real Chinese candidates.
2. Replace sample dictionary output with the built-in pinyin mapping.
3. Add failing engine tests for Chinese commit behavior.
4. Update engine expectations if needed to keep fallback behavior explicit.
5. Add failing TSF host tests that assert Chinese commit requests.
6. Verify registration scripts and manual Windows flow.

## Risks

- The TSF host may still expose gaps in composition-to-editor integration even after dictionary work is complete.
- Some Windows editors may behave differently under TSF; Notepad is the primary verification target for the MVP.
- Encoding issues may surface in PowerShell output or registry display names, but they are secondary to actual Chinese text commit behavior.

## Non-Goals for This Iteration

Do not spend time in this MVP on:

- large dictionaries
- learning models
- UI polish
- schema management
- service/database integration for live typing
- cross-platform abstractions beyond what already exists

## Acceptance Checklist

- [ ] Built-in pinyin dictionary returns Chinese candidates
- [ ] Engine tests prove known pinyin codes commit Chinese text
- [ ] TSF host tests prove Chinese commit requests are generated
- [ ] Development install script registers the IME locally
- [ ] Manual test in Notepad confirms real Chinese commit
- [ ] Unknown inputs do not crash the session
