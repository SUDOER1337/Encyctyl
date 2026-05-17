# Encyctyl

**Dev-native knowledge system.** Markdown-first, offline-first, keyboard-first, git-native, Graph view, local AI agent.

> **Status:** Pre-MVP / early scaffolding. Workspace, vault, parser, and SQLite metadata crates exist. No UI, no search, no sync yet.

## Stack

- **Core:** Rust workspace
- **Desktop (planned):** egui + eframe + wgpu
- **Mobile (planned):** Kotlin + Jetpack Compose, Rust via JNI
- **Storage:** Markdown files (source of truth) + SQLite metadata
- **Search (planned):** Tantivy full-text + vector embeddings
- **AI (optional / planned):** Local inference via llama.cpp / Ollama

## Crates

| Crate | Status | What it does |
|---|---|---|
| `core` | done | Shared types, config, error enum |
| `vault` | done | File discovery, atomic read/write |
| `parser` | done | Markdown → frontmatter, wiki-links, tags |
| `storage` | done | SQLite metadata (tags, links, backlinks) |
| *(more planned)* | — | graph, indexer, search, git_sync, ai_runtime... |

## Quick start

```bash
cargo run                           # indexes .md files in current directory
cargo run path/to/vault             # point at a specific vault directory
cargo build
cargo check -p encyctyl-vault --features indexer
```

Full roadmap is in [`plan.md`](./plan.md).

## Constraints

- No telemetry by default
- No mandatory cloud
- No plugin ecosystem
- No webviews on Android
- Target startup: desktop <400ms / Android <1.5s
- Target idle RAM: <200MB desktop
