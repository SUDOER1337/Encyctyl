# Encyctyl — AGENTS.md

## Identity
- **Remote**: `git@github.com:SUDOER1337/Encyctyl.git`
- **Status**: Early scaffolding — core, vault, and parser crates implemented. Pre-MVP, no tests yet.
- **Goal**: Dev-native knowledge system — plaintext-first, offline-first, keyboard-first, git-native, local AI.

## Architecture (planned — see `dev_native_knowledge_system_agent_plan.md`)
- **Stack**: Rust core → egui desktop + Kotlin/Compose Android
- **Storage**: Markdown files (source of truth) + SQLite metadata
- **Search**: Tantivy full-text + vector embeddings (bge-small/nomic-embed/all-MiniLM)
- **AI**: Local inference via llama.cpp / Candle / ONNX Runtime
- **Sync**: Git-native (shell git first, eventually gitoxide)
- **IPC**: Rust ↔ Kotlin via JNI (minimal crossings, coarse-grained APIs)

## Current repo structure
```
repo/
├── apps/       (empty)
├── crates/     core/ [x] vault/ [x] parser/ [x]
├── mobile/     rust-ffi/
├── docs/ scripts/ tools/
```

(Remaining crates: graph, indexer, search, git_sync, embeddings, rag, ai_runtime, workspace, whiteboard, vim, storage, ipc, events, telemetry — still to build.)

## Development progress (from plan)
1. Vault system          [x]
2. Markdown parser       [x]
3. SQLite metadata       [ ]
4. Search/indexing       [ ]
5. egui editor           [ ]
6. Graph engine          [ ]
7. Git sync              [ ]
8. Android integration   [ ]
9. AI runtime            [ ]
10. Whiteboard           [ ]

## Example Projects (reference only — NOT part of the project)
- `Example Projects/terax-ai/` — Tauri 2 + Rust + React AI terminal (reference for Tauri/PTY patterns)
- `Example Projects/whiteboard/` — Logseq fork, Android-first git-synced notes (reference for note-taking architecture)
- **These directories are gitignored.** Do not import, modify, or build from them.

## Constraints
- No telemetry by default, no mandatory cloud, no plugin ecosystem, no webviews on Android, no Electron-style overhead
- Startup targets: desktop <400ms / Android <1.5s; idle RAM <200MB desktop
- MVP avoids: collaborative editing, cloud sync, theme ecosystem, whiteboard
