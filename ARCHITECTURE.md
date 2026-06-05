# Architecture — Polychora Temporal

> *A genuine 4D voxel world simulation stack. The engine renders four spatial dimensions natively using Vulkan, with a layered architecture separating world simulation, rendering backends, plugin content, and network multiplayer.*

## Design Goals

1. **Native 4D** — W is a real spatial dimension, not a trick. Every component (storage, traversal, rendering, physics) operates in 4D.
2. **Pluggable rendering** — Multiple backends for comparison, debugging, and research (VTE, tetra raytrace, tetra raster).
3. **Server-authoritative multiplayer** — Clients are views into a shared 4D world state.
4. **WASM content plugins** — Third-party blocks, entities, and behaviors without modifying engine code.

## High-Level Overview

```
┌──────────────────────────────────────────────────────────────────┐
│                    POLYCHORA TEMPORAL                             │
│                                                                  │
│  ┌────────────────────┐    ┌──────────────────────────────┐      │
│  │  polychora (crate) │───▶│  Rendering Backends          │      │
│  │  - Game client      │    │  ┌────────┐ ┌──────────┐   │      │
│  │  - Server binary    │    │  │  VTE   │ │Tetra RT  │   │      │
│  │  - World field      │    │  └────────┘ └──────────┘   │      │
│  │  - Mob simulation   │    │  ┌──────────┐              │      │
│  │  - Save/load (v4)   │    │  │Tetra Rast│              │      │
│  └────────┬───────────┘    │  └──────────┘              │      │
│           │                └──────────────────────────────┘      │
│           │                                                      │
│  ┌────────▼───────────┐    ┌──────────────────────────────┐      │
│  │polychora-content    │    │  Temporal Bridge             │      │
│  │(WASM plugin)       │◀──▶│  ┌────────────────┐         │      │
│  │- Blocks & entities │    │  │ TemporalWorld   │         │      │
│  │- Mob behavior      │    │  │ EventVoxel      │         │      │
│  │- Procgen           │    │  │ Conservation    │         │      │
│  └────────────────────┘    │  │ Tracker         │         │      │
│                            │  └────────────────┘         │      │
│  ┌────────────────────┐    └──────────────────────────────┘      │
│  │polychora-plugin-api│                                          │
│  │(Stable ABI crate)  │                                          │
│  └────────────────────┘                                          │
│                                                                  │
│  ┌────────────────────────────────────────────────────┐          │
│  │  src/ + slang-shaders/ (Core rendering infra)      │          │
│  │  - Vulkan device setup, shader pipelines           │          │
│  │  - Hypercube geometry, capture, HUD                │          │
│  │  - BVH, texture pools, profiler                    │          │
│  └────────────────────────────────────────────────────┘          │
└──────────────────────────────────────────────────────────────────┘
```

## Core Components

### polychora (main crate)

**Purpose:** Game client, dedicated server, shared world model, voxel storage, content registry, save/load, mob simulation.

**Key modules:**
- `server/` — Authoritative server, mob simulation, world field
- `shared/` — Region tree, render tree, WASM loading
- `voxel/` — 4D voxel chunk storage and operations
- `migration/` — Save format migration (v3→v4)
- `save_v4/` — Binary world persistence format
- `block_gui/`, `content_registry/`, `builtin_content/`, `plugin_loader/`

### Rendering Infrastructure (src/)

**Purpose:** Vulkan device management, pipeline creation, shader compilation, HUD rendering, screenshot capture.

**Key modules:**
- `render/vte.rs` — Voxel Traversal Engine: native 4D DDA ray casting through chunk/cell data
- `render/capture.rs` — GPU screenshot capture
- `render/pipelines.rs` — Shader pipeline compilation and caching
- `render/hud.rs`, `render/overlay.rs` — Text and UI overlay
- `render/bvh_topology_tests.rs` — BVH for tetra geometry

### polychora-plugin-api

**Purpose:** Stable ABI crate for WASM content plugins. Defines types, traits, and opcodes for block/entity behavior, procgen, GUI rendering, and texture definitions.

### polychora-content

**Purpose:** First-party WASM content plugin. Implements built-in blocks, entities, mob AI, and procedural generators.

### polychora-temporal-bridge

**Purpose:** Extends the engine with W→Time integration — temporal world state, event voxels, conservation tracking for the ternary fleet integration.

## Data Flow

```
User Input (keyboard/mouse)
       │
       ▼
Game Client (polychora)
       │
       │  Local/Server ──► Server (polychora-server)
       │       │                   │
       │       ▼                   ▼
       │  World Field ────►  Region Tree (shared state)
       │       │                   │
       │       ▼                   ▼
       │  4D Voxel Chunks ←───  Replication
       │       │
       ▼       ▼
Rendering Backend (VTE / Tetra RT / Tetra Rast)
       │
       ▼
Display Output (window or headless PNG)
```

## Key Design Decisions

### Two-Stage 4D Rendering (VTE)

- **Context:** Rendering 4D voxels to a 2D screen requires projecting through W
- **Chosen approach:** Stage A casts rays through 4D producing a 3D hyper-image; Stage B resolves it to 2D (integral, slice, thick-slice modes)
- **Trade-offs:** More memory for intermediate buffer, but enables debug/comparison modes and clean separation of concerns

### Server-Authoritative World

- **Context:** Multiplayer consistency in a 4D world
- **Chosen approach:** Server owns world state, sends streaming subtree updates
- **Trade-offs:** Higher latency for state edits vs P2P, but guarantees consistency

### WASM Plugin ABI

- **Context:** Allow user-defined content without recompiling engine
- **Chosen approach:** Stable `#[no_std]` ABI crate with explicit opcodes and serialized data
- **Trade-offs:** Serialization overhead vs. direct FFI, but sandboxed and portable

## Dependencies

| Dependency | Why We Use It | Notes |
|-----------|---------------|-------|
| `vulkano` | Safe Vulkan bindings | 0.35 |
| `winit` | Window creation and event loop | 0.30 |
| `glam` | Math types with bytemuck | 4D vectors, bytemuck for GPU transfer |
| `ndarray` | N-dimensional array operations | In common crate |
| `nalgebra` | Linear algebra | In common crate |
| `image` | Screenshot I/O | With rayon for parallelism |
| `ab_glyph` | Text rendering | HUD and overlays |
| `exr` | EXR image format | For HDR output |

## Extension Points

- **New rendering backend** — Implement a new module following the VTE/tetra pattern, register in the backend selector
- **WASM content plugin** — Implement `polychora-plugin-api` traits, compile to WASM, ship with manifest
- **Procedural generator** — Add to `procgen/` in `polychora-content`
- **Mob behavior** — Implement mob AI logic via the plugin system

## See Also

- [GETTING_STARTED.md](./GETTING_STARTED.md) — Build and run
- [LOW_LEVEL.md](./LOW_LEVEL.md) — Internal details
- [API_REFERENCE.md](./API_REFERENCE.md) — Full API
- [docs/](./docs/) — Detailed design docs, specs, and plans
