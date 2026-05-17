# Encyctyl

**Dev-native knowledge system.** Plaintext-first, offline-first, keyboard-first, git-native, local AI.

> **Status:** Pre-MVP / early scaffolding. Workspace, vault, parser, and SQLite metadata crates exist. No UI, no search, no sync yet.

## Stack

- **Core:** Rust workspace
- **Desktop (planned):** egui + eframe + wgpu
- **Mobile (planned):** Kotlin + Jetpack Compose, Rust via JNI
- **Storage:** Markdown files (source of truth) + SQLite metadata
- **Search (planned):** Tantivy full-text + vector embeddings
- **AI (planned):** Local inference via llama.cpp / Candle / ONNX

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
