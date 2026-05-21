# RENAME MIGRATION: Zynta Studio → LocalFlow IDE

## Summary

- **Old name**: Zynta Studio
- **New name**: LocalFlow IDE
- **Old npm scope**: `@zynta/*`
- **New npm scope**: `@local-flow/*`
- **Old Rust crate**: `zynta-studio`
- **New Rust crate**: `localflow-ide`
- **Old bundle identifier**: `com.zyntastudio.dev`
- **New bundle identifier**: `com.localflow.ide`

## Files Changed

### Package Manifests

| File | Change |
|------|--------|
| `package.json` | `name: "local-flow-ide"`, all deps `@local-flow/*` |
| `apps/desktop/package.json` | `name: "@local-flow/desktop"`, all deps `@local-flow/*` |
| `packages/logging/package.json` | `name: "@local-flow/logging"`, dep `@local-flow/shared-types` |
| `packages/runtime-contracts/package.json` | `name: "@local-flow/runtime-contracts"`, dep `@local-flow/shared-types` |
| `packages/shared-types/package.json` | `name: "@local-flow/shared-types"` |
| `packages/state/package.json` | `name: "@local-flow/state"`, deps `@local-flow/*` |
| `packages/ui/package.json` | `name: "@local-flow/ui"`, dep `@local-flow/shared-types` |

### Rust

| File | Change |
|------|--------|
| `src-tauri/Cargo.toml` | `name = "localflow-ide"` |
| `src-tauri/Cargo.lock` | Regenerated on next build |

### Tauri Configuration

| File | Change |
|------|--------|
| `src-tauri/tauri.conf.json` | `productName: "LocalFlow IDE"`, `identifier: "com.localflow.ide"`, window title: "LocalFlow IDE" |
| `src-tauri/capabilities/default.json` | description updated |

### Frontend Assets

| File | Change |
|------|--------|
| `apps/desktop/index.html` | `<title>LocalFlow IDE</title>` |
| `public/logo-full.svg` | Text updated to "LOCALFLOW IDE" |

### Source Files (imports)

14 files with `@zynta/*` import paths updated to `@local-flow/*`:
- `apps/desktop/src/App.tsx`
- `apps/desktop/src/components/*.tsx` (7 files)
- `apps/desktop/src/hooks/useRuntimeEvents.ts`
- `packages/state/src/*.ts` (3 files)
- `packages/logging/src/*.ts` (2 files)

### Display Strings

| File | Change |
|------|--------|
| `apps/desktop/src/components/StatusBar.tsx` | "Zynta Studio" → "LocalFlow IDE" |
| `apps/desktop/src/components/MainArea.tsx` | "Zynta Studio" → "LocalFlow IDE" |
| `apps/desktop/src/components/panels/ExtensionsPanel.tsx` | "Zynta Studio" → "LocalFlow IDE" |
| `src-tauri/src/main.rs` | log messages updated |
| `packages/logging/src/logger.ts` | source: 'zynta' → source: 'localflow' |

### Infrastructure

| File | Change |
|------|--------|
| `kubernetes/namespace.yaml` | `name: local-flow-ide` |
| `kubernetes/service.yaml` | `name: local-flow-ide-service`, `namespace: local-flow-ide` |
| `kubernetes/deployment.yaml` | app labels, names, namespace all updated |
| `kubernetes/ingress.yaml` | service ref, namespace updated |
| `kubernetes/hpa.yaml` | name, namespace updated |
| `kubernetes/canary.yaml` | name, namespace, app labels updated |
| `vercel.json` | `name: "local-flow-ide"` |

### Documentation

| File | Change |
|------|--------|
| `CONTRIBUTING.md` | References updated to @local-flow/* |
| `verification-report.md` | Header updated |
| `findings.md` | References updated |
| `progress.md` | Project name updated |
| `task_plan.md` | References updated |
| `DEPLOYMENT_GUIDE.md` | Marked for rewrite (old marketing content) |

## Post-Migration Steps

1. Delete `node_modules` and `pnpm-lock.yaml`, run `pnpm install`
2. Delete `src-tauri/target`, run `cargo check` (fresh build)
3. Verify `pnpm build` succeeds
4. Verify `pnpm typecheck` passes
5. Verify `pnpm lint` passes

## Remaining Work

- README.md — full rewrite (part of Step 8)
- DEPLOYMENT_GUIDE.md — delete or rewrite with accurate content
- Old build artifacts in `src-tauri/target/release/bundle/` still reference ZyntaStudio in .msi/.exe names (clean build fixes)
