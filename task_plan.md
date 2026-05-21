# LocalFlow IDE — Full Architecture Refactoring Plan

## Mission
Refactor this repository from a buzzword-heavy prototype into a production-grade local-first AI development environment with real runtime systems, proper state management, responsive UI, and engineering discipline.

## Architecture Principles
1. **No fake orchestration** — if it says it runs, it must actually run
2. **UI renders state, never simulates it** — frontend consumes runtime events only
3. **Strict domain boundaries** — runtime, UI, and contracts are separate packages
4. **Observability by default** — every operation is traced and logged
5. **Contracts at every boundary** — typed IPC, Zod validation, Serde schemas

## Phases

### Phase 1 — Monorepo Architecture [IN PROGRESS]
- pnpm workspace setup
- Directory restructuring: /apps, /packages, /src-tauri
- Shared types package
- Build tooling configuration

### Phase 2 — Real Runtime System [PENDING]
- Rust process supervisor with Tokio
- Async task queue with lifecycle management
- Event bus for runtime events
- Typed IPC contracts (request/response schemas)
- Execution tracing and persistence

### Phase 3 — State Management [PENDING]
- Zustand stores replacing scattered useState
- Normalized state graph
- Typed action/event system
- Runtime event subscription layer

### Phase 4 — Terminal System [PENDING]
- Real PTY integration (portable-pty)
- stdout/stderr streaming
- Process lifecycle management
- Command history and persistence

### Phase 5 — Responsive UI [PENDING]
- Remove glassmorphism + excessive glow
- Mobile-first responsive design
- Proper typography hierarchy
- Collapsible panels
- Accessibility improvements

### Phase 6 — Performance Engineering [PENDING]
- Code splitting + lazy loading
- Render optimization
- Bundle analysis
- Virtualization where needed

### Phase 7 — Security & Sandboxing [PENDING]
- Command allowlists
- Filesystem boundaries
- Zod validation on all IPC
- Runtime permission controls

### Phase 8 — Observability [PENDING]
- Structured telemetry pipeline
- Execution tracing
- Error boundaries
- Runtime health monitoring

### Phase 9 — Documentation [PENDING]
- Rewrite README: remove buzzwords, add real architecture docs
- IPC contract documentation
- Developer onboarding guide
- Execution model docs

### Phase 10 — Engineering Discipline [PENDING]
- ESLint strict rules + Prettier
- Vitest for frontend tests
- Rust tests (cargo test)
- CI pipeline configuration
- Bundle budget enforcement

## Errors Log
| Phase | Error | Attempt | Resolution |
|-------|-------|---------|------------|
| - | - | - | - |

## Comparative Audit: 2026-05-21

### Goal
Compare LocalFlow IDE against VS Code, opencode, and Agent Zero, then produce a blunt improvement roadmap focused on product credibility, working condition, and differentiation.

### Phases
- [complete] Inspect current LocalFlow IDE structure, runtime wiring, tests, and docs
- [complete] Inspect reference project positioning and architecture
- [complete] Run local build/test/type checks where practical
- [complete] Synthesize the gap assessment and top-tier roadmap
