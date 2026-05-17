
# Dev-Native Knowledge System — Full Multi-Agent Architecture Plan
Version: 1.0
Target Stack: Rust Core + egui Desktop + Kotlin/Compose Android

---

# 1. Vision

A minimal, opinionated, ultra-fast developer-oriented knowledge system combining:

- Markdown notes
- Journaling
- Knowledge graph
- Whiteboard/canvas
- Git-native workflow
- Vim-first editing
- Local AI assistant
- RAG-style retrieval
- Offline-first architecture

The product intentionally avoids:
- Plugin marketplaces
- Heavy theming
- Cloud lock-in
- Browser-first architecture
- Electron-style memory overhead

Core design philosophy:

> Plaintext forever. Local-first. Git-native. Keyboard-native. Deterministic UX.

---

# 2. High-Level Product Goals

## Primary Goals

1. Instant startup
2. Extremely low memory usage
3. Native-feeling Android experience
4. Git-first synchronization
5. Fast indexing/search
6. Strong offline support
7. Local AI support
8. Predictable UX
9. No plugin ecosystem
10. Minimal operational complexity

---

# 3. Architectural Overview

```text
                 ┌────────────────────┐
                 │  Rust Core Engine  │
                 │--------------------│
                 │ Markdown parser    │
                 │ Graph engine       │
                 │ Git sync           │
                 │ Fulltext search    │
                 │ Embeddings / RAG   │
                 │ Workspace manager  │
                 │ Cache/index        │
                 └─────────┬──────────┘
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
┌───────▼────────┐ ┌───────▼────────┐
│ Desktop UI     │ │ Android UI     │
│ egui + wgpu    │ │ Compose        │
└────────────────┘ └────────────────┘
```

---

# 4. Repository Structure

```text
repo/
├── apps/
│   ├── desktop/
│   └── android/
│
├── crates/
│   ├── core/           [x]
│   ├── vault/          [x]
│   ├── parser/         [x]
│   ├── graph/
│   ├── indexer/
│   ├── search/
│   ├── git_sync/
│   ├── embeddings/
│   ├── rag/
│   ├── ai_runtime/
│   ├── workspace/
│   ├── whiteboard/
│   ├── vim/
│   ├── storage/
│   ├── ipc/
│   ├── events/
│   └── telemetry/
│
├── mobile/
│   └── rust-ffi/
│
├── docs/
├── scripts/
└── tools/
```

---

# 5. Technology Decisions

## Core Language

Rust

Reason:
- predictable performance
- low memory usage
- async support
- excellent tooling
- portability
- ideal for indexing/search/AI runtime

---

# 6. UI Architecture

# Desktop UI Agent

## Responsibilities

- egui frontend
- GPU rendering
- layout system
- docking/panes
- graph visualization
- whiteboard rendering
- command palette
- modal editing integration

## Recommended Stack

- egui
- eframe
- wgpu
- tokio
- egui_dock
- egui_extras

## UI Principles

- keyboard-first
- low animation usage
- deterministic layout
- instant interactions
- no web technologies

---

# 7. Android Agent

## Responsibilities

- Jetpack Compose frontend
- native Android UX
- IME handling
- gesture support
- file permissions
- Rust FFI integration

## Recommended Stack

- Kotlin
- Jetpack Compose
- Android NDK
- JNI bridge
- Rust shared libraries

## Critical Rules

1. Android text editing must be native
2. Scrolling must remain 60fps+
3. Avoid webviews entirely
4. Rust owns business logic
5. Compose owns platform UX

---

# 8. Core Engine Agent

## Responsibilities

- application orchestration
- workspace lifecycle
- indexing pipeline
- synchronization scheduling
- event system
- task coordination

## Internal Systems

```text
Core Engine
├── Workspace Manager
├── Task Scheduler
├── Event Bus
├── File Watcher
├── Cache Layer
├── Index Coordinator
└── AI Coordinator
```

---

# 9. Storage Agent

## Source of Truth

Markdown files.

## Metadata

SQLite.

## Database Responsibilities

Store:
- backlinks
- tags
- graph edges
- embeddings
- search index metadata
- workspace cache
- whiteboard state
- note history
- AI chunk references

## Suggested Libraries

- rusqlite
- sqlite-vss
- serde
- bincode

---

# 10. Vault Agent [x]

## Responsibilities

- workspace loading
- note discovery
- path normalization
- attachment handling
- atomic writes
- conflict detection

## Vault Rules

1. Never corrupt markdown
2. Atomic file writes only
3. File-first architecture
4. Human-readable forever
5. Metadata must be recoverable

---

# 11. Markdown Parser Agent [x]

## Responsibilities

- markdown parsing
- wiki-link extraction
- tag extraction
- block references
- frontmatter parsing
- AST generation

## Recommended Libraries

- pulldown-cmark
- markdown-it-rs
- tree-sitter

## Output

```text
Markdown File
    ↓
AST
    ↓
Semantic Nodes
    ↓
Graph/Index Pipeline
```

---

# 12. Graph Engine Agent

## Responsibilities

- backlink management
- graph generation
- node relationships
- graph traversal
- semantic clustering

## Data Model

```text
Node
├── id
├── title
├── path
├── tags
├── backlinks
├── embeddings
└── metadata

Edge
├── source
├── target
├── type
└── weight
```

## Rendering Goals

- incremental rendering
- lazy loading
- GPU acceleration
- stable layouts
- low memory overhead

---

# 13. Whiteboard Agent

## Responsibilities

- infinite canvas
- note nodes
- edge rendering
- zoom/pan
- freeform organization

## Constraints

1. Never become Figma
2. Lightweight interactions only
3. Focus on knowledge relationships
4. Fast startup priority

---

# 14. Search Agent

## Responsibilities

- full-text search
- fuzzy matching
- symbol indexing
- semantic retrieval
- ranking

## Recommended Stack

- Tantivy
- fst
- unicode-normalization

## Features

- instant search
- incremental indexing
- low-latency ranking
- offline operation

---

# 15. Git Synchronization Agent

## Responsibilities

- repository detection
- auto-commit
- pull/push orchestration
- merge handling
- conflict resolution

## Initial Strategy

Use shell git commands first.

Later:
- migrate to gitoxide

## Sync Rules

1. Never silently overwrite
2. Pull before push
3. Explicit conflict surfaces
4. Offline-safe operations

---

# 16. Vim Engine Agent

## Responsibilities

- modal editing
- motions
- operators
- command mode
- text objects

## MVP Scope

```text
hjkl
w/b/e
gg/G
dd
yy
p
u
/
:
```

## Non-Goals

- full Vim compatibility
- Vimscript
- plugin support

---

# 17. AI Runtime Agent

## Responsibilities

- model loading
- inference orchestration
- prompt assembly
- token streaming
- context management

## Supported Backends

- llama.cpp
- Candle
- ONNX Runtime

## Design Rules

1. Local-first AI
2. Optional cloud support later
3. Stream responses immediately
4. Quantized models preferred

---

# 18. Embeddings Agent

## Responsibilities

- chunking
- embedding generation
- semantic indexing
- cache invalidation

## Pipeline

```text
Markdown
    ↓
Chunker
    ↓
Embedding Model
    ↓
Vector Storage
    ↓
Retrieval
```

## Suggested Models

- bge-small
- nomic-embed
- all-MiniLM

---

# 19. RAG Agent

## Responsibilities

- retrieval
- reranking
- context windows
- citation linking
- note-aware prompts

## Retrieval Pipeline

```text
User Query
    ↓
Hybrid Search
├── BM25
└── Vector Search
    ↓
Reranking
    ↓
Context Assembly
    ↓
LLM
```

---

# 20. Indexing Agent

## Responsibilities

- incremental indexing
- background workers
- dependency tracking
- workspace refreshes

## Critical Requirements

1. Never block UI thread
2. Prioritize active notes
3. Incremental updates only
4. Debounced file watching

---

# 21. Event Bus Agent

## Responsibilities

- application-wide messaging
- decoupled architecture
- async coordination

## Suggested Stack

- tokio broadcast
- crossbeam
- flume

---

# 22. IPC / FFI Agent

## Responsibilities

- Rust/Kotlin bridge
- serialization
- thread safety
- async communication

## Rules

1. Rust owns state
2. Kotlin owns UI lifecycle
3. Minimize JNI crossings
4. Use coarse-grained APIs

---

# 23. Performance Agent

## Responsibilities

- memory profiling
- startup optimization
- render profiling
- indexing benchmarks

## Hard Targets

### Desktop
- startup < 400ms
- idle RAM < 200MB

### Android
- cold startup < 1.5s
- scrolling 60fps
- battery-safe background work

---

# 24. Security Agent

## Responsibilities

- sandbox AI execution
- protect local data
- secure model loading
- safe file handling

## Rules

1. No telemetry by default
2. No mandatory cloud
3. Local data remains local
4. Explicit permission prompts

---

# 25. Testing Agent

## Responsibilities

- integration testing
- workspace stress tests
- sync testing
- corruption prevention
- indexing verification

## Required Test Suites

- large vault tests
- git conflict tests
- Android lifecycle tests
- malformed markdown tests
- embedding cache tests

---

# 26. Build/Release Agent

## Responsibilities

- CI/CD
- cross-platform builds
- Android packaging
- release signing

## Recommended Stack

- GitHub Actions
- cargo-nextest
- cargo-chef
- Android Gradle

---

# 27. Telemetry Agent

## Philosophy

Telemetry must be:
- opt-in
- anonymous
- minimal

## Metrics

- startup time
- crash reports
- indexing duration
- render latency

---

# 28. MVP Roadmap

# Phase 1 — Core Foundation

Goals:
- vault loading          [x]
- markdown editing       [x] (parser only, no editor yet)
- SQLite metadata
- file watching
- basic search

Deliverables:
- desktop alpha
- note editor
- backlinks
- command palette

---

# Phase 2 — Graph + Git

Goals:
- graph visualization
- git sync
- journal system
- tag explorer

Deliverables:
- graph view
- sync UI
- note linking

---

# Phase 3 — Android

Goals:
- Compose frontend
- Rust core integration
- mobile sync

Deliverables:
- mobile editor
- search
- git pull/push

---

# Phase 4 — AI/RAG

Goals:
- local embeddings
- semantic retrieval
- assistant chat

Deliverables:
- AI sidebar
- contextual retrieval
- local inference

---

# Phase 5 — Whiteboard

Goals:
- infinite canvas
- graph interaction
- semantic grouping

Deliverables:
- canvas mode
- node linking
- drag interactions

---

# 29. Scaling Strategy

## 10K+ Notes

Requirements:
- incremental indexing
- lazy graph loading
- virtualized rendering
- chunked embeddings

---

# 30. Non-Goals

Do NOT build:
- Notion clone
- collaborative editor
- plugin marketplace
- theme ecosystem
- browser-first app
- cloud-first architecture

---

# 31. Engineering Principles

1. Plaintext first
2. Offline first
3. Keyboard first
4. Git native
5. Minimal dependencies
6. Native rendering
7. Predictable performance
8. No unnecessary abstraction
9. Fast startup over flashy UI
10. Long-term maintainability

---

# 32. Suggested Initial Crates

```toml
tokio
serde
serde_json
rusqlite
tantivy
notify
pulldown-cmark
tree-sitter
crossbeam
flume
egui
eframe
wgpu
rayon
anyhow
thiserror
tracing
```

---

# 33. Recommended Development Order

1. Vault system          [x]
2. Markdown parser       [x]
3. SQLite metadata
4. Search/indexing
5. egui editor
6. Graph engine
7. Git sync
8. Android integration
9. AI runtime
10. Whiteboard

---

# 34. Final Product Positioning

## Intended Audience

- developers
- researchers
- technical writers
- PKM power users
- terminal/Vim users
- offline-first users

## Product Identity

> A fast, local-first developer knowledge system with Git-native workflows and integrated local AI.

---

# 35. Final Technical Recommendation

The strongest long-term architecture is:

- Rust core
- egui desktop
- Kotlin Compose Android
- SQLite metadata
- Markdown source of truth
- Tantivy search
- Local embeddings
- Git-native sync
- Local AI inference

This maximizes:
- performance
- maintainability
- scalability
- offline capability
- developer experience
- future AI extensibility
