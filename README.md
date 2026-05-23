# Zynta Studio

**Local-first AI development environment.**

A Rust + Tauri desktop application that provides a cognitive runtime for autonomous engineering workflows. Features process execution, terminal management, task scheduling, resource governance, and an architecture knowledge graph — all running locally.

## Architecture

```
┌────────────────────────────────────────┐
│            Frontend (React)            │
├────────────────────────────────────────┤
│         Tauri IPC Command Bridge       │
├────────────────────────────────────────┤
│  Scheduler  ·  Supervisor  ·  Engine   │
│  PTY Manager  ·  Sandbox  ·  Governor │
│  Brain (Arch Graph + Exec DAG)        │
│  Model Router  ·  Memory Store        │
│  Runtime Inspector  ·  Telemetry      │
├────────────────────────────────────────┤
│          OS Layer (Win/Linux/Mac)      │
└────────────────────────────────────────┘
```

## Key Subsystems

- **Execution Graph** — Task DAG with node types, topological ordering, and lineage tracing.
- **Architecture Graph** — Static codebase analyzer mapping imports, calls, and dependencies.
- **Task Scheduler** — Priority-queued task dispatch with worker limits.
- **Supervisor** — Concurrent task lifecycle management (running, completed, failed, timed out).
- **Process Runner** — Async subprocess execution with stdout/stderr capture.
- **PTY Terminal** — Full terminal session management via `portable-pty`.
- **Resource Governor** — Token-based memory/parallelism limits.
- **Sandbox** — Path validation and command allowlisting.
- **Model Router** — Multi-provider LLM inference (Ollama, OpenAI, Anthropic) with health checks.
- **Memory Store** — On-disk persistence for key-value state.
- **Telemetry** — Structured logging, metrics collection, cost tracking, and timeline analysis.

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Frontend | React, Tailwind CSS |
| Backend | Rust, Tokio, Tauri 2 |
| Terminal | `portable-pty` |
| IPC | Tauri JSON-RPC commands |
| Serialization | Serde / Serde JSON |
| LLM Integration | reqwest (Ollama, OpenAI, Anthropic) |

## Getting Started

```bash
# Install dependencies
cd src-tauri
cargo build

# Run in development mode
cargo tauri dev
```

## License

GPL-3.0
