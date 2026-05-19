# Contributing to Zynta Studio

## Setup

```bash
# Install pnpm
npm install -g pnpm@latest

# Install all workspace dependencies
pnpm install

# Type-check all packages
pnpm typecheck

# Run linting
pnpm lint

# Run tests
pnpm test
```

## Architecture

The project is a pnpm monorepo with:

- `apps/desktop/` — Tauri desktop app (React frontend + Rust backend)
- `packages/` — Shared packages consumed by the desktop app

Key packages:
- `@zynta/shared-types` — Core types (Task, Event, Terminal)
- `@zynta/runtime-contracts` — Zod schemas for IPC validation
- `@zynta/state` — Zustand stores
- `@zynta/ui` — Shared React components
- `@zynta/logging` — Structured logging

## Backend (Rust)

```bash
# Type-check Rust
cargo check

# Run Rust tests
cargo test

# Build release
cargo build --release
```

## Frontend

```bash
# Dev server
pnpm --filter @zynta/desktop dev

# Build for production
pnpm --filter @zynta/desktop build
```

## Adding a new IPC command

1. Add the command in `src-tauri/src/ipc/commands.rs` using `#[tauri::command]`
2. Register it in `build_handler()` function
3. Add the Zod schema in `packages/runtime-contracts/src/`
4. Add the TypeScript types in `packages/shared-types/src/`
5. Add the Zustand store action in `packages/state/src/`
6. Connect in frontend via `useRuntimeEvents` hook

## Design tokens

All design tokens live in `apps/desktop/src/styles.css` as CSS custom properties under `:root`.

- Colors: `--bg-*`, `--text-*`, `--accent-*`
- Typography: `--font-mono`, `--font-sans`
- Spacing: `--space-*` (4px increments)
- Radii: `--radius-*`
- Transitions: `--transition-*`

## Adding tests

- Frontend: Vitest in `apps/desktop/src/**/*.test.ts`
- Rust: `#[cfg(test)]` modules in `src-tauri/src/**`

## Commit conventions

- `feat:` new feature
- `fix:` bug fix
- `refactor:` code restructure without behavior change
- `chore:` tooling, deps, config
- `docs:` documentation only

## Code style

- TypeScript: ESLint + Prettier (run `pnpm format` to auto-fix)
- Rust: `cargo fmt` + `cargo clippy`