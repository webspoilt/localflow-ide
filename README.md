# Zynta Studio

**A local-first development environment that runs AI tasks on your own machine.**

Zynta Studio is a desktop application for executing, orchestrating, and observing development workflows — with a real process supervisor, sandboxed command execution, and a terminal that actually runs commands.

## What it does

- **Task execution** — Submit commands to a Rust-based process supervisor that runs them locally with configurable timeouts, retries, and sandbox policies
- **Terminal** — Interactive xterm.js terminal that communicates with the Rust backend over typed IPC
- **File explorer** — Open workspace folders, navigate the file tree, and open files
- **State management** — Zustand stores for runtime tasks, terminal sessions, workspace, and UI — all synced via Tauri's event system
- **Security** — Command denylist, dangerous pattern detection, sandbox policies enforced server-side

## Architecture

```
┌──────────────────────────────────────────────────────┐
│                  React Frontend                       │
│  Activity Bar │ Sidebar │ Main Area │ Terminal      │
│       │        │         │           │               │
│       └────────┴─────────┴───────────┴───── Tauri IPC │
│                    (invoke / listen)                  │
├──────────────────────────────────────────────────────┤
│              Rust Backend (Tokio async)               │
│  Task Queue │ Process Runner │ Sandbox │ Events       │
│  Telemetry  │ Storage        │ Git    │ LLM          │
└──────────────────────────────────────────────────────┘
```

## Stack

| Layer | Technology |
|-------|-----------|
| UI | React 18 + TypeScript 5 |
| State | Zustand |
| Terminal | xterm.js |
| Backend | Rust + Tokio |
| Desktop | Tauri v2 |
| Build | Vite + pnpm |

## Getting started

```bash
# Install dependencies
pnpm install

# Frontend dev
pnpm dev

# Full desktop app
cd src-tauri && cargo tauri dev

# Build
pnpm build
```

## Project structure

```
apps/desktop/          Tauri desktop app
  src/
    components/        UI components (ActivityBar, Sidebar, Terminal, etc.)
    hooks/             Custom hooks (useRuntimeEvents)
    styles.css         Design tokens + global styles
packages/
  shared-types/        Core TypeScript types (Task, Event, Terminal)
  runtime-contracts/  Zod validation schemas for IPC
  state/              Zustand stores
  ui/                Shared UI primitives
  logging/           Structured logging
src-tauri/
  src/
    core/             Task queue, process supervisor, lifecycle
    ipc/              Tauri command handlers
    process/         Process runner (tokio::process), terminal manager
    sandbox/          Command allowlist, pattern detection
    events/           Runtime event definitions
    storage/          Task history persistence
    telemetry/        tracing + metrics
    git/              Git operations
    llm/              Ollama integration
```

## IPC Commands

| Command | Purpose |
|---------|---------|
| `health` | Runtime status check |
| `execute_task` | Queue a background task |
| `execute_command` | Run a shell command immediately |
| `read_directory` | List directory contents |
| `create_terminal` | Open a terminal session |
| `cancel_task` | Cancel a running task |

## Security

- Server-side command validation blocks `rm -rf /`, `sudo`, `dd`, and other dangerous commands
- Pattern detection for malicious shell constructs
- All IPC inputs validated with Zod schemas before processing

## Design principles

- **No fake execution** — If the UI says something runs, it actually runs
- **UI renders state** — Frontend consumes runtime events, never simulates them
- **Typed at every boundary** — TypeScript types shared across packages, Zod schemas on IPC
- **Minimal animations** — Every animation has a purpose (state change feedback, not decoration)
- **Clarity over style** — Premium feel through restraint, not effects

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for setup instructions.

## License

GPL-3.0