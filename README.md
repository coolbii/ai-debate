# Agora Debate Theater

Nx monorepo scaffold for the AI Oregon-style debate system.

## Workspace layout

```text
apps/
  web/          React frontend
  api/          Rust backend
packages/
  shared-types/
  prompt-spec/
  debate-protocol/
  ui/
tools/
  scripts/
```

## Quick start

```bash
pnpm install
pnpm dev:web
```

## Rust backend

Install Rust (`cargo`) before running:

```bash
pnpm dev:api
```
