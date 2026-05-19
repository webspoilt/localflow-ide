# Zynta Studio

**Local-first AI development environment.**

A desktop application for executing, orchestrating, and observing AI-assisted development workflows — all running locally on your machine.

## Architecture

```
┌──────────────────────────────────────────────────────────┐
│                     Frontend (React)                      │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌─────────┐ │
│  │ Activity │  │  Editor  │  │ Terminal │  │ Status  │ │
│  │   Bar    │  │  Panels  │  │   (xterm)│  │  Bar    │ │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬────┘ │
│       │              │              │              │      │
│  ┌────┴──────────────┴──────────────┴──────────────┴──┐  │
│  │               Zustand State Stores                  │  │
│  │  RuntimeStore │ TerminalStore │ WorkspaceStore      │  │
│  └───────────────────────┬─────────────────────────────┘  │
└──────────────────────────┼───────────────────────────────┘
                           │ Tauri IPC (typed, validated)
┌──────────────────────────┼───────────────────────────────┐
│    ┌─────────────────────┴────────────────────────┐      │
│    │            Tauri Commands Layer               │      │
│    │  health │ execute_task │ terminal_*            │      │
│    └─────────────────────┬─────────────────────────┘      │
│                          │                                 │
│  ┌───────────────────────┴─────────────────────────────┐  │
│  │              Rust Runtime Core                       │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐           │  │
│  │  │  Task    │  │ Process  │  │ Sandbox  │           │  │
│  │  │  Queue   │  │ Runner   │  │ Policy   │           │  │
│  │  ├──────────┤  ├──────────┤  ├──────────┤           │  │
│  │  │  Event   │  │ Terminal │  │ Storage  │           │  │
│  │  │  Bus     │  │ Manager  │  │ History  │           │  │
│  │  └──────────┘  └──────────┘  └──────────┘           │  │
│  │  ┌────────────────────────────────────────┐          │  │
│  │  │  Telemetry (tracing-subscriber + JSON)  │          │  │
│  │  └────────────────────────────────────────┘          │  │
│  └──────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────┘
```

### Domain Boundaries

| Package | Responsibility |
|---------|---------------|
| `@zynta/shared-types` | Core type definitions (Task, Event, Terminal, Runtime contracts) |
| `@zynta/runtime-contracts` | Zod validation schemas for all IPC boundaries |
| `@zynta/state` | Zustand stores (Runtime, Terminal, Workspace, UI) |
| `@zynta/ui` | Shared React UI primitives |
| `@zynta/logging` | Structured logging pipeline |
| `src-tauri/core` | Task queue, process supervisor, lifecycle management |
| `src-tauri/ipc` | Tauri command handlers with sandbox validation |
| `src-tauri/process` | Process runner (tokio::process), terminal manager |
| `src-tauri/sandbox` | Command allowlists, filesystem boundaries |
| `src-tauri/events` | Runtime event types (Serde-serializable) |
| `src-tauri/storage` | Persistent task history store |
| `src-tauri/telemetry` | Structured JSON logging, metrics collection |

## Runtime Model

### Task Lifecycle

1. **Queued** — Task submitted via IPC, enters FIFO queue
2. **Running** — Process supervisor picks up task, spawns OS process
3. **Completed/Failed** — Process exits, stdout/stderr captured, event emitted
4. **Cancelled** — Task removed from queue or process aborted

Tasks support:
- Configurable timeouts
- Automatic retry with exponential backoff
- Sandbox validation before execution
- Structured event emission at every lifecycle stage

### Event Bus

All runtime events flow through a typed channel (`mpsc::UnboundedSender<RuntimeEvent>`):
- Task lifecycle events (created, queued, started, completed, failed, cancelled)
- Terminal session events (created, output, closed)
- Runtime health events (started, shutdown, health check)

Frontend subscribes via Tauri's `listen()` API and dispatches to Zustand stores.

## Getting Started

### Prerequisites

- Node.js >= 18
- pnpm >= 8
- Rust >= 1.77
- Tauri CLI v2

### Install

```bash
pnpm install
```

### Development

```bash
# Frontend only (Vite dev server)
pnpm dev

# Full Tauri desktop app
cd src-tauri
cargo tauri dev
```

### Build

```bash
# Frontend production build
pnpm build

# Tauri production build
cd src-tauri
cargo tauri build
```

### Type Checking

```bash
pnpm typecheck       # Frontend TypeScript
cd src-tauri && cargo check  # Rust
```

### Testing

```bash
pnpm test            # Vitest (frontend)
cd src-tauri && cargo test  # Rust tests
```

## Project Structure

```
├── apps/
│   └── desktop/          # Tauri desktop application
│       ├── src/          # React frontend source
│       │   ├── components/    # UI components
│       │   │   └── panels/    # Sidebar panels
│       │   └── hooks/         # React hooks
│       └── index.html
├── packages/
│   ├── shared-types/     # Core type definitions
│   ├── runtime-contracts/ # Zod validation schemas
│   ├── state/            # Zustand state management
│   ├── ui/               # Shared UI components
│   └── logging/          # Structured logging
├── src-tauri/            # Rust backend
│   ├── src/
│   │   ├── core/         # Task queue, supervisor, lifecycle
│   │   ├── ipc/          # Tauri command handlers
│   │   ├── process/      # Process runner, terminal
│   │   ├── sandbox/      # Security policy enforcement
│   │   ├── events/       # Event type definitions
│   │   ├── storage/      # Data persistence
│   │   ├── telemetry/    # Tracing and metrics
│   │   ├── git/          # Git operations
│   │   └── llm/          # LLM integration (Ollama)
│   └── Cargo.toml
├── pnpm-workspace.yaml
└── tsconfig.base.json
```

## IPC Contracts

All IPC calls are validated server-side with Zod schemas:

| Command | Request | Response |
|---------|---------|----------|
| `health` | — | `{ status, version, uptime }` |
| `execute_task` | `{ command, cwd? }` | `{ task_id, status }` |
| `cancel_task` | `{ task_id }` | `bool` |
| `create_terminal` | `{ cwd? }` | `session_id` |
| `close_terminal` | `{ session_id }` | `bool` |
| `list_terminals` | — | `[session_id]` |

## Security Model

- **Command allowlisting**: Server-side validation blocks dangerous commands (rm -rf /, sudo, etc.)
- **Sandbox policies**: Configurable filesystem and network access boundaries
- **Zod validation**: All IPC inputs validated at the Tauri command layer

## Telemetry

- Structured JSON logging via `tracing-subscriber`
- File and console output
- Runtime metrics: task counts, execution time, active workers
- All events carry source, severity, timestamp, and correlation IDs

## License

GPL-3.0
