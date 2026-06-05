# PLUG_AND_PLAY — Polychora Temporal

> **A genuine 4D voxel game engine and multiplayer sandbox — build, explore, and simulate in four spatial dimensions.**

## What Is This?

Polychora Temporal is a full 4D voxel world simulation stack built on Vulkan. It's a temporal fork extending the original Polychora engine with W→Time integration for the ternary fleet. Players move through X/Y/Z/W space, edit geometry that's impossible in 3D, and interact with a server-authoritative multiplayer world with mob simulation.

## Why Should You Care?

- **True 4D rendering** — Not faked slices. Native 4D voxel traversal, tetrahedron ray tracing, and tetra rasterization backends.
- **Multiplayer sandbox** — Server-authoritative world editing with streaming replication and mob AI in 4D.
- **WASM plugin system** — Content plugins with stable ABI for blocks, entities, mob behavior, and procedural generation.
- **Temporal bridge** — W-axis as both space and time coordinate, enabling conservation tracking and temporal world features.

## Quick Start

```bash
cargo build --release
# Interactive 4D explorer:
cargo run --release
```

## ✨ Key Features

- Chunked 4D voxel world (8×8×8×8 chunks) with real-time editing
- Three rendering backends: Voxel Traversal Engine, Tetra Ray Tracing, Tetra Rasterization
- Procedural world generation with mazes and structure blueprints
- Server-authoritative multiplayer with mob pathfinding in 4D
- WASM content plugin ABI for extensibility
- Temporal world bridge with event voxels and conservation tracking

## Next Steps

| Guide | What It Covers |
|-------|----------------|
| [`GETTING_STARTED.md`](./GETTING_STARTED.md) | Build, run, first-time setup |
| [`ARCHITECTURE.md`](./ARCHITECTURE.md) | How it works under the hood |
| [`API_REFERENCE.md`](./API_REFERENCE.md) | Key crates, types, and public API surface |
| [`LOW_LEVEL.md`](./LOW_LEVEL.md) | Internals, performance, porting guide |

## Status

**Active development.** The W→Time temporal bridge is experimental. Rendering backends are production-grade for the VTE path.
