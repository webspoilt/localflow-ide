# Findings — Zynta Studio Codebase Analysis

## Critical Issues Identified

### 1. Fake Runtime Simulation
- `AgentTerminal.tsx` uses `setInterval` to cycle through hardcoded mock logs
- No actual subprocess execution in the terminal panel
- The Rust backend has real git/VFS/LLM code but none of it is wired to the UI

### 2. No State Management
- Zero shared state — every component uses local `useState`
- No React Context, no Zustand, no Redux
- Components cannot communicate through any state layer

### 3. Build Errors in Rust
- `metrics` crate API mismatch (`counter!()` vs function call)
- `auth.rs`: `Vec` used where `HashSet` expected
- `auction.rs`: borrow-after-move

### 4. Overengineered Marketing, Underengineered Implementation
- README claims tree-sitter, git2, quantum circuits — none exist
- "Time Travel Debugger" has Rust ring buffer but UI is fake
- "Provenance Blockchain" is a 182-line SHA-256 chain — real but disconnected

### 5. UI Anti-Patterns
- Glassmorphism with `backdrop-filter: blur(24px)` everywhere — GPU heavy
- Fixed viewport layouts with `overflow: hidden` — breaks on resize
- Hardcoded dark theme — no light mode support
- No responsive breakpoints

### 6. No Testing Infrastructure
- Zero test files in frontend or backend
- No CI pipeline configuration
- No linting beyond TypeScript strict mode

## Architectural Decisions

### Why pnpm Workspaces?
- Strict dependency isolation prevents phantom dependencies
- Faster installs with content-addressable storage
- Built-in workspace protocol for monorepo development

### Why Zustand over XState?
- Lower boilerplate for our use case
- Better TypeScript inference
- Smaller bundle size
- Easier to integrate with Tauri event system
- XState's finite state machine model adds complexity without proportional benefit for this app

### Why portable-pty over xterm.js?
- xterm.js is a terminal emulator (frontend rendering)
- portable-pty is a PTY allocator (backend process management)
- We need BOTH: portable-pty for process execution, xterm.js for rendering

### Why Tokio for Runtime?
- Tauri already uses Tokio as its async runtime
- Native async task spawning and cancellation
- Ecosystem: tracing, tokio-util, tokio-stream

## Technology Stack
- **Runtime**: Rust + Tokio + Tauri v2
- **Frontend**: React 18 + TypeScript 5 + Vite 6
- **State**: Zustand
- **Terminal**: portable-pty + xterm.js (via @xterm/xterm)
- **Validation**: Zod
- **Testing**: Vitest + @testing-library/react + cargo test
- **Linting**: ESLint + Prettier
- **Build**: pnpm workspaces + Turborepo (optional)
