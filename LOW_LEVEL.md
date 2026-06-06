# LOW LEVEL — Polychora Temporal

> *For engine contributors, performance tuners, and anyone porting to new platforms. This document dives into internal module structure, key implementation patterns, and optimization strategies.*

## Internal Architecture

### Crate Structure

```
polychora-temporal/
├── src/                          # Core rendering infrastructure
│   ├── lib.rs                    # Module declarations
│   ├── hypercube.rs              # 4D camera, projection, rotation
│   ├── matrix_operations.rs      # 4D linear algebra
│   ├── render/                   # Rendering subsystem
│   │   ├── vte.rs                # Voxel Traversal Engine
│   │   ├── capture.rs            # GPU screenshot capture
│   │   ├── texture_pool.rs       # Texture atlas management
│   │   ├── hud.rs                # Text overlay rendering
│   │   ├── pipelines.rs          # Shader pipeline compilation
│   │   ├── types.rs              # Shared render types (VteDisplayMode, RenderBackend, etc.)
│   │   ├── geometry.rs           # Geometry buffers
│   │   ├── bvh_topology_tests.rs # BVH for tetra tracing
│   │   └── ...
│   └── vulkan_setup.rs           # Vulkan device & instance
├── common/                       # Shared N-dimensional math (Semantics4D, VecN, MatN)
├── crates/
│   ├── polychora/                # Main game client + server
│   │   ├── src/
│   │   │   ├── server/           # Server + mob sim
│   │   │   ├── shared/           # Region tree, render tree, WASM
│   │   │   ├── voxel/            # 4D voxel storage
│   │   │   └── ...
│   ├── polychora-content/        # First-party WASM plugin
│   ├── polychora-plugin-api/     # Stable plugin ABI
│   ├── polychora-temporal-bridge/# W→Time integration (TemporalWorld, EventVoxel, TemporalGlue, ConservationTracker)
│   ├── polychora-room-runtime/   # Room/multiplayer runtime (Room, Tile, Ecology, Provenance)
│   ├── demo/                     # Demo binary
│   └── exr-converter/            # EXR frame converter
└── slang-shaders/                # Slang shader source files
```

### Module Map

| Module | Responsibility | Key Types |
|--------|---------------|-----------|
| `render::vte` | Voxel Traversal Engine — 4D DDA ray cast | `VteDisplayMode`, `GpuVoxelFrameMeta`, `VteDebugCounters` |
| `shared::region_tree` | Spatial region tree for world replication | `RegionTree`, `RegionNode` |
| `shared::wasm` | WASM runtime loading and execution | `WasmPlugin` |
| `server::mob_sim` | 4D mob pathfinding and behavior | `Mob`, `PathFinder` |
| `voxel/` | 4D chunked voxel storage (8×8×8×8 chunks) | `Chunk`, `VoxelWorld` |

## Key Internal Patterns

### VTE Two-Stage Rendering

The Voxel Traversal Engine separates 4D→2D rendering into two pipelined stages:

1. **Stage A (4D→3D hyper-image):** Casts rays through 4D voxel space using 4D DDA (Digital Differential Analyzer) with native chunk/cell traversal. Rays have 4D direction vectors and intersect 4D voxels directly. Output is a 3D buffer (width × height × layers).

2. **Stage B (3D→2D display operator):** Resolves the 3D hyper-image to 2D output. Four modes:
   - `integral` — Sum samples along W for full projection
   - `slice` — Sample a single W layer
   - `thick-slice` — Sample a range of W layers
   - `debug-compare` — Side-by-side comparison with reference trace
   - `debug-integral` — Integral mode with diagnostic overlay
   - `temporal-trace` — Highlight temporal trail through time layers
   - `temporal-arrow` — Show time direction as glyphs on temporal voxels
   - `temporal-spacetime` — Minkowski-style spacetime diagram projection
   - `temporal-event` — Highlight event voxels along the temporal axis

```rust
// Pseudocode for VTE pipeline
fn render_frame(world: &VoxelWorld, camera: &Camera4D) -> Frame {
    let hyper_image = stage_a_cast_4d_rays(world, camera);
    stage_b_resolve_3d_to_2d(hyper_image, display_mode)
}
```

### Server-Authoritative World Replication

The server owns the authority for the world state. Clients send edit requests; the server validates, applies, and broadcasts updates. World state is organized as a region tree — subtrees are streamed to clients based on their position in 4D space.

## Performance

### Benchmarks

Benchmarks are ad-hoc (run with `cargo run --release` and observe FPS). Key performance characteristics:

- **VTE backend:** ~30-60 FPS on mid-range GPU at 1080p. Performance scales with `--layers` (the W depth).
- **Tetra raytrace:** ~5-15 FPS — path tracing each tetra intersection is expensive.
- **Tetra raster:** ~15-30 FPS — faster than raytrace but lower quality.

### Hot Paths

- **VTE ray casting inner loop** — The inner DDA traversal is the single hottest path. Each pixel casts one 4D ray, stepping through voxel cells. Optimized with integer arithmetic and branchless step logic.
- **Voxel chunk access** — 4D chunk indexing is O(1) via flat array offset computation.
- **WASM plugin calls** — Plugin calls cross the WASM boundary; hot callbacks (block tick, entity tick) should minimize ABI crossings.

### Allocation Profile

- Chunk allocations are pre-allocated (each chunk is 8×8×8×8 = 4096 voxels, fixed size).
- Frame allocations: VTE allocates the hyper-image buffer once per resize, reuses across frames.
- WASM instances allocate within their linear memory; engine-side serialization copies.

## Concurrency & Thread Safety

- Vulkan commands are submitted from the main thread (vulkano requirement).
- Server tick runs on a dedicated thread.
- Mob simulation runs tick-by-tick on the server thread.
- WASM plugins are single-threaded (per instance) — no Send/Sync guarantees across plugin boundaries.
- Shared world state is protected by RwLock (read from render, write from server tick).

## Error Handling

- Vulkan errors are panicked (abort) — these indicate unrecoverable GPU/driver failures.
- World load/save returns `Result<(), SaveError>` — I/O errors are propagated.
- WASM plugin load errors return descriptive error messages with plugin name.
- Network protocol errors disconnect the client gracefully.

## Testing

### Unit Tests

Run with `cargo test`. Tests exist in:
- `crates/polychora/src/shared/region_tree/` — Region tree serialization
- `src/render/bvh_topology_tests.rs` — BVH topology correctness
- `common/` — Math utilities

### Integration Tests

- `cargo run -p demo --release -- --headless` — End-to-end rendering test (outputs PNG)
- Server-client test: Run server + client locally, verify connection and world state sync

## Debugging

- **VTE debug env vars:** `R4D_VTE_REFERENCE_COMPARE=1` enables reference trace comparison
- **Screenshot metadata:** F12 captures print camera pose, backend, and VTE mode to stdout
- **Validation layers:** Enable Vulkan validation layers at build time

## Porting Guide

### Adding a New Rendering Backend

1. Implement the render interface in `src/render/<name>.rs`
2. Add variant to `BackendType` enum
3. Register in the backend selector (`src/render/mod.rs`)
4. Add `--backend <name>` CLI flag handling
5. Update `GETTING_STARTED.md` with new backend option

### Adding a New Platform

| Platform | Notes |
|----------|-------|
| Linux | Primary platform. Tested with NVIDIA, AMD, Intel GPUs |
| macOS | Through MoltenVK (Vulkan→Metal translation layer). Slang compiler needed |
| Windows | Should work with Vulkan SDK for Windows. Test coverage limited |
| WASM | Not currently supported (Vulkan required). Future: WebGPU target |

### Adding WASM Content

1. Create a new crate depending on `polychora-plugin-api`
2. Implement plugin traits (block definitions, entity behavior, procgen)
3. Compile to WASM: `cargo build --target wasm32-wasip1 --release`
4. Write a manifest file describing your blocks and entities
5. Place in `plugins/` directory; engine loads automatically

## Future Work

- Performance: GPU-based VTE compute shader (currently CPU ray cast)
- Non-voxel entities: Integration of mesh-based objects alongside voxels
- Multi-scale LOD: Distant horizon rendering for large worlds
- WebGPU/WASM target: Browser-based 4D exploration
- Save format evolution: v4 → v5 with compression and streaming
