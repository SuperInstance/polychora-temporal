# API Reference — Polychora Temporal

> *Public API surface of the primary crates. Built for plugin authors, integrators, and engine contributors. This reference covers the main crate modules and the plugin ABI. MSRV: stable (see `rust-toolchain.toml`).*

---

## `higher_dimension_playground` (src/)

### `hypercube`

```rust
pub mod hypercube;
```

4D geometry primitives — camera, projection, rotation math across all six spatial planes (XY, XZ, XW, YZ, YW, ZW).

**Key items:**
- `Camera` — 4D camera with position (Vec4), basis vectors, projection matrix
- `Hypercube` — Wireframe geometry for debug visualization of 4D shapes

### `matrix_operations`

```rust
pub mod matrix_operations;
```

4D linear algebra utilities — matrix construction, decomposition, transformation helpers used by the rendering pipeline.

### `render`

```rust
pub mod render;
```

Core rendering subsystem. Sub-modules for Vulkan setup, pipeline compilation, HUD, capture, VTE, tetra backends.

**Key sub-modules:**
- `render::vte` — Voxel Traversal Engine backend
- `render::capture` — GPU screenshot capture
- `render::pipelines` — Shader pipeline compilation
- `render::types` — Shared render types and enums (`VteDisplayMode`, `RenderBackend`, `RenderOptions`)

### `vulkan_setup`

```rust
pub mod vulkan_setup;
```

Vulkan device creation, instance management, swapchain setup, and event loop integration.

---

## `polychora` (main game crate)

### Entry Points

```rust
// Game client binary
// polychora --release [OPTIONS]

// Dedicated server binary
// polychora-server --release --bind <addr> [OPTIONS]
```

### `content_registry`

```rust
pub mod content_registry;
```

Registry for content plugins. Loads WASM plugins, manages block/entity definitions, texture mappings.

**Key types:**
- `ContentRegistry` — Holds all registered blocks, entities, textures
- `ContentPlugin` — Loaded WASM plugin handle

### `block_gui`

```rust
pub mod block_gui;
```

Block interaction GUI — inventory management, block placement UI, hotbar.

### `save_v4`

```rust
pub mod save_v4;
```

Binary world save/load in `.v4dw` format. Chunk-level serialization of 4D voxel data.

### `server`

```rust
pub mod server;
```

Server-authoritative multiplayer subsystem.

**Key modules:**
- `server::mob_sim` — 4D mob pathfinding, line-of-sight, collision, abilities
- `server::world_field` — World field simulation (field effects, region influence)

**Key types:**
- `Server` — Polychora dedicated server
- `WorldField` — Field-level world operations

### `shared`

```rust
pub mod shared;
```

Shared state between client and server, including tree structures for world replication.

**Key modules:**
- `shared::region_tree` — Spatial region tree for world state organization
- `shared::render_tree` — Render data tree derived from world state
- `shared::wasm` — WASM runtime integration

### `shared::region_tree`

```rust
pub mod region_tree;
```

Region-based spatial tree for organizing the 4D world. Subtree replication enables streaming world updates.

**Key types:**
- `RegionTree` — Root tree structure
- `RegionNode` — Single node in the tree
- `RegionKey` — Spatial key identifying a region

---

## `polychora-plugin-api` (WASM plugin ABI)

```rust
#![no_std]
extern crate alloc;
```

Stable ABI for content plugins. All types are `#[repr(C)]` compatible.

### `block`

```rust
pub mod block;
```

Block definition types — block state, block ID, material properties.

**Key types:**
- `BlockId` — Unique block identifier
- `BlockState` — Block metadata (health, orientation, custom data)
- `BlockDefinition` — Full block specification

### `block_tick_abi`

```rust
pub mod block_tick_abi;
```

ABI for per-tick block behavior. Blocks can define `tick` callbacks for periodic updates.

### `entity`

```rust
pub mod entity;
```

Entity types — mobs, items, projectiles.

**Key types:**
- `EntityId` — Unique entity identifier
- `EntityDefinition` — Entity specification
- `EntityState` — Entity runtime state (position, velocity, health)

### `entity_tick_abi`

```rust
pub mod entity_tick_abi;
```

ABI for per-tick entity behavior — movement, AI, interaction callbacks.

### `content_ids`

```rust
pub mod content_ids;
```

Pre-assigned IDs for built-in blocks and entities.

### `manifest`

```rust
pub mod manifest;
```

Plugin manifest format — declares blocks, entities, textures, and dependencies.

### `procgen_abi`

```rust
pub mod procgen_abi;
```

ABI for procedural world generation plugins — structure placement, terrain shaping.

### `texture`

```rust
pub mod texture;
```

Texture definition and lookup — pixel data, UV mapping, texture atlases.

### `opcodes`

```rust
pub mod opcodes;
```

Operation codes for internal plugin IPC — serialized ABI calls.

### `side_effects`

```rust
pub mod side_effects;
```

Side-effect system for block/entity interactions — explosions, area effects, field modifications.

### `model_abi`

```rust
pub mod model_abi;
```

ABI for 4D mesh/model definitions — vertex data for tetrahedra and hypercubes.

### `gui_abi`

```rust
pub mod gui_abi;
```

ABI for plugin-provided GUI rendering — inventory screens, dialogs, tooltips.

### `region_tree`

```rust
pub mod region_tree;
```

Region tree types shared between engine and plugins for spatial queries.

---

## `polychora-content` (first-party WASM plugin)

First-party content implementing built-in blocks, entities, and procedural generators through the plugin API. Compiled to WASM and loaded at runtime.

**Key items:**
- `procgen/` — Terrain generation, maze generation, structure placement
- Built-in block types for the default world
- Mob behavior implementations for default entity types

---

## `polychora-temporal-bridge` (W→Time integration)

### `TemporalWorld`

```rust
pub struct TemporalWorld { ... }
```

W-axis temporal integration — maps W coordinate to time dimension, tracks state evolution over time.

### `EventVoxel`

```rust
pub struct EventVoxel { ... }
```

A voxel that records time-domain events — block changes, entity interactions, world events.

### `TemporalGlue`

```rust
pub struct TemporalGlue { ... }
```

Glue layer bridging the temporal world model with the main polychora engine.

### `ConservationTracker`

```rust
pub struct ConservationTracker { ... }
```

Tracks conservation of properties (mass, energy, information) across temporal operations.

### `TemporalGlue`

```rust
pub struct TemporalGlue { ... }
```

Auto-wiring infrastructure that detects `Semantics4D::Temporal` and wires up the temporal crate ecosystem: conservation tracking, time advance, and diagnostics.

---

## `polychora-room-runtime` (Room / ecology runtime)

### `Room`

```rust
pub struct Room { ... }
```

A scoped gameplay context inside the temporal 4D world. Contains tiles, agents, a frozen context snapshot for temporal stability, and metadata.

### `Tile`

```rust
pub struct Tile { ... }
```

A bounded region with presets, a creator context hash, required agents, and capabilities flags for temporal gameplay.

### `Ecology`

```rust
pub struct Ecology { ... }
```

Multi-agent coordination runtime: manages rooms, handles split/merge operations at tile boundaries, and tracks active rooms.

### `Provenance`

```rust
pub struct Provenance { ... }
```

Tracks origin and modification history of tiles and rooms — creator identity, timestamps, and action records.

---

## `common` — Shared Math and Semantics

### `Semantics4D`

```rust
pub enum Semantics4D {
    Spatial,  // W = 4th spatial axis
    Temporal, // W = time dimension
}
```

Controls whether the 4th axis (W) is treated as a spatial dimension or a time dimension. Passed via `--semantics` CLI flag.

### `VteDisplayMode`

```rust
pub enum VteDisplayMode {
    Integral,
    Slice,
    ThickSlice,
    DebugCompare,
    DebugIntegral,
    TemporalTrace,      // Trace through time layers
    TemporalArrow,      // Show time direction as glyphs
    TemporalSpacetime,  // Minkowski diagram projection
    TemporalEvent,      // Highlight event voxels
}
```

Controls the Stage-B display operator for the Voxel Traversal Engine. Passed via `--vte-display-mode` CLI flag.

## Feature Gates

| Feature | What It Enables | Default? |
|---------|----------------|----------|
| `vte-diagnostics` | Debug-only VTE comparison and diagnostics env vars | No |

## Re-exports

- `common` — Shared ndarray/nalgebra math used by CPU and GPU paths (features: `ndarray`, `nalgebra`)

## Minimum Supported Rust Version (MSRV)

Stable Rust. Pinned by `rust-toolchain.toml`.
