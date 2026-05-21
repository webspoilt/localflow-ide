# Verification Report — LocalFlow IDE Phase 1

Generated: 2026-05-21
Checks: `pnpm typecheck` `pnpm lint` `pnpm test` `cargo check` `cargo test --no-run`

---

## 1. `pnpm typecheck` — FAILED (14 errors)

| Error | Count | Root Cause |
|-------|-------|------------|
| TS6305: Output `.d.ts` not built from source | 14 | `tsconfig.json` has `composite: true` but `tsc --noEmit` cannot verify without building project references first |

**Affected files:** All `.tsx` files in `apps/desktop/src/`
**Confidence:** High — composite requires explicit build step before typecheck
**Remediation:** Add `"build": "tsc -b"` step before typecheck, or remove `composite` from `tsconfig.json`

---

## 2. `pnpm lint` — FAILED (89 errors, 1 warning)

| Category | Count | Root Cause |
|----------|-------|------------|
| `no-confusing-void-expression` | 21 | Arrow function shorthands returning `void` (e.g. `onClick={() => setState(val)`) |
| `no-unnecessary-condition` | 14 | Non-nullish values used with `??` operator |
| `no-unused-vars` | 10 | Imports and variables declared but never used |
| `restrict-template-expressions` | 7 | `number` type in template literal without explicit string conversion |
| `no-floating-promises` | 2 | `setup()` called without `await` or `.catch()` |
| `no-misused-promises` | 3 | Async functions passed to `onClick` expecting `() => void` |
| `no-unnecessary-type-assertion` | 2 | Assertions on already-correct types |
| `no-empty-function` | 2 | Empty arrow functions as event handlers |
| ESLint config parse error | 2 | `vite.config.ts` and `vitest.config.ts` not in `tsconfig.json` include |
| Other | 26 | `no-dynamic-delete`, `array-type`, `no-unsafe-call`, `require-await`, etc. |

**Affected files:**
- `apps/desktop/src/App.tsx` — unused imports
- `apps/desktop/src/components/ActivityBar.tsx` — void expression
- `apps/desktop/src/components/EditorTabs.tsx` — void expression
- `apps/desktop/src/components/MainArea.tsx` — unused imports
- `apps/desktop/src/components/Sidebar.tsx` — unused import
- `apps/desktop/src/components/StatusBar.tsx` — unused imports, template types
- `apps/desktop/src/components/TerminalPanel.tsx` — unused import, floating promise, unnecessary condition
- `apps/desktop/src/components/panels/ExplorerPanel.tsx` — unused imports, misused promises, empty fn
- `apps/desktop/src/components/panels/SearchPanel.tsx` — unused import, array type
- `apps/desktop/src/components/panels/SettingsPanel.tsx` — unused imports, void expression
- `apps/desktop/src/hooks/useRuntimeEvents.ts` — `prefer-const`, floating promise, void expression
- `packages/logging/src/logger.ts` — console.log warning, void expression
- `packages/logging/src/formatters.ts` — template expression types
- `packages/state/src/runtime-store.ts` — unnecessary conditions, no-dynamic-delete, void expression
- `packages/state/src/terminal-store.ts` — unused vars, unnecessary condition, void expression
- `packages/state/src/ui-store.ts` — void expression
- `packages/state/src/workspace-store.ts` — void expression
- `vite.config.ts` (root) — unsafe call, require-await

**Confidence:** Very high — all 89 errors reproducible
**Remediation:** Fix lint violations across all files; add `vite.config.ts` files to tsconfig

---

## 3. `pnpm test` — FAILED (cannot load vite)

| Error | Root Cause |
|-------|------------|
| `ERR_MODULE_NOT_FOUND`: Cannot find package 'vite' | Root-level `vite.config.ts` imports `vite` but `vite` is only installed in `apps/desktop/` |

**Affected files:** Root `vite.config.ts`, `package.json` (test script)
**Confidence:** Very high
**Remediation:** Move test infrastructure to app level or install vite at root; update root `package.json` test script

---

## 4. `cargo check` — FAILED (build script error)

| Error | Root Cause |
|-------|------------|
| `Permission dialog:default not found` | `dialog` plugin referenced in `capabilities/default.json` but `tauri-plugin-dialog` not in `Cargo.toml` dependencies |

**Additional issues discovered:**
- `#[warn(unused)]` — 38 warnings for unused struct fields, unused imports, dead code

**Affected files:**
- `src-tauri/Cargo.toml` — missing `tauri-plugin-dialog`
- `src-tauri/capabilities/default.json` — references non-existent plugin
- `src-tauri/src/core/supervisor.rs` — `_task_queue` parameter unused
- `src-tauri/src/process/terminal.rs` — `event_sender` field never sends events
- `src-tauri/src/storage/task_history.rs` — dead code (never instantiated)
- `src-tauri/src/telemetry/metrics.rs` — `MetricsSnapshot` never constructed
- `src-tauri/src/llm/` — two files, one with same content

**Confidence:** Very high
**Remediation:** Add `tauri-plugin-dialog = "2"` to Cargo.toml; fix warnings by removing dead code or wiring it in

---

## 5. `cargo test --no-run` — FAILED (same root cause as cargo check)

Same `dialog:default` permission error prevents compilation.

---

## Summary: System Health

| Check | Status | Blockers |
|-------|--------|----------|
| TypeScript typecheck | **FAIL** | composite mode requires build step |
| ESLint | **FAIL** | 89 rule violations |
| Test suite | **FAIL** | vitest cannot load at root level |
| Rust build | **FAIL** | missing tauri-plugin-dialog crate |
| Rust test compile | **FAIL** | same dialog dependency missing |
| Rust warnings | **WARN** | 38 unused/dead code warnings |

---

## Documentation vs Reality Audit

| README Claim | Reality | Verdict |
|-------------|---------|---------|
| "real process supervisor" | `Supervisor::run()` loops forever doing nothing | **FAKE** |
| "sandboxed command execution" | Only blocks known-dangerous commands; allowlist not enforced | **PARTIAL** |
| "Zod schemas on IPC" | Schemas defined in `runtime-contracts` but never used in IPC handlers | **UNWIRED** |
| "interactive xterm.js terminal" | Terminal accepts input but runs one-shot `execute_command` per line | **PARTIAL** |
| "state synced via Tauri event system" | `useRuntimeEvents` hook listens but events are never emitted beyond `RuntimeStarted` | **UNWIRED** |
| "Terminal: multi-session support" | `TerminalManager` creates sessions but PTY is never spawned | **FAKE** |
| "command history" | `terminal-store` has `commandHistory` but nothing writes to it | **UNWIRED** |
| "Ollama integration" | `llm.rs` module exists but is never called from any command | **DEAD CODE** |
| "Git operations" | `git/mod.rs` has functions but they're never exposed via IPC | **DEAD CODE** |
| "Task history persistence" | `TaskHistoryStore` never instantiated | **UNWIRED** |
| "Runtime metrics" | `MetricsCollector` never instantiated | **UNWIRED** |
| "CSP security" | CSP header allows localhost:1420 but production builds need Tauri's default | **PARTIAL** |

---

## Fake Systems Identified (Phase 2 Target)

1. **ExplorerPanel mock data** — When `read_directory` IPC fails, fallback creates synthetic `src/app.tsx`, `src/index.ts`, etc.
2. **StatusBar hardcoded values** — `main` branch hardcoded, `Zynta v0.2.0` (formerly) hardcoded
3. **Terminal "Done" labels** — `[Done]` / `[Exit X]` are UI-generated decorations, not runtime state
4. **Supervisor::run() empty loop** — No task dequeue, no execution, no event emission
5. **cancel_task returns false** — Never cancels anything
6. **portable-pty in Cargo.toml, unused** — Dependency present but `terminal.rs` doesn't use it
7. **LLM module** — Dead code, no IPC handler calls it
8. **Git module** — Dead code, no IPC handler references it
9. **TaskHistoryStore** — Dead code, never constructed
10. **MetricsCollector** — Dead code, never constructed
11. **Event system** — Events defined, sender wired, but only `RuntimeStarted` ever emitted
12. **Zod schemas** — Defined but never used; IPC commands accept raw strings only

---

## Recommended Order of Remediation

| Priority | Action | Phase |
|----------|--------|-------|
| P0 | Add `tauri-plugin-dialog` to Cargo.toml | 1 |
| P0 | Fix `cancel_task` to actually cancel | 2 |
| P0 | Implement `Supervisor::run()` with real dequeue+execute cycle | 4 |
| P0 | Wire event system — emit real events from supervisor | 4 |
| P0 | Remove mock folder data from ExplorerPanel | 2 |
| P1 | Fix TS6305 — remove composite or add build step | 8 |
| P1 | Fix all 89 ESLint violations | 8 |
| P1 | Remove dead code: git, llm modules (or wire them) | 2 |
| P1 | Wire Zod schemas into IPC handlers | 4 |
| P2 | Implement real PTY with portable-pty | 3 |
| P2 | Add streaming terminal output via Tauri events | 3 |
| P2 | Wire TaskHistoryStore and MetricsCollector into lifecycle | 4 |
| P3 | Fix CI pipeline to run on master not main | 8 |
| P3 | Update README to reflect only implemented features | 7 |
| P3 | Add test files (zero exist) | 8 |
