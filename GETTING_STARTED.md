# GETTING STARTED — Polychora Temporal

> *Estimated time to complete: 10 minutes*

## Prerequisites

- **Rust toolchain** (stable, see `rust-toolchain.toml`)
- **Vulkan SDK** with validation layers
- **Slang compiler** and SPIR-V tools
- Cargo

## Installation

### From Source

```bash
git clone https://github.com/SuperInstance/polychora-temporal.git
cd polychora-temporal
cargo build --release
```

### Vulkan SDK Setup

On Arch Linux:
```bash
yay -S shader-slang-bin spirv-tools vulkan-devel
```

On Ubuntu/Debian:
```bash
sudo apt install spirv-tools
# Download Slang from https://github.com/shader-slang/slang/releases
```

On macOS:
```bash
brew install shader-slang spirv-tools
```

## Your First 10 Minutes

### 1. Launch the 4D Explorer

```bash
cargo run --release
```

You'll see a 4D voxel world. Controls:
- **W/A/S/D** — Move in 3D
- **Q/E** — Move along the W axis (4th dimension)
- **Mouse** — Look around
- **Left click** — Break blocks
- **Right click** — Place blocks

### 2. Try Different Backends

```bash
cargo run --release -- --backend voxel-traversal
cargo run --release -- --backend tetra-raytrace
cargo run --release -- --backend tetra-raster
```

The `voxel-traversal` backend is the primary native 4D renderer.

### 3. Run the Demo

```bash
cargo run -p demo --release -- --headless
```

This renders a single frame to PNG without opening a window.

### 4. Start a Multiplayer Server

```bash
# Terminal 1: Server
cargo run -p polychora --bin polychora-server --release -- --bind 0.0.0.0:4000

# Terminal 2: Client connecting
cargo run --release -- --server 127.0.0.1:4000 --player-name "Player1"
```

## Common Patterns

### Exploring with Different Display Modes (VTE)

```bash
# See the hidden dimension as integrated samples
cargo run --release -- --vte-display-mode integral

# View a single slice of the W dimension
cargo run --release -- --vte-display-mode slice --vte-slice-layer 4

# Debug: compare backends side-by-side
cargo run --release -- --vte-display-mode debug-compare
```

### Temporal Mode with the W→Time Bridge

```bash
# Enable temporal W-axis semantics
cargo run --release -- --semantics temporal

# See temporal trail through time layers
cargo run --release -- --vte-display-mode temporal-trace

# Minkowski spacetime diagram projection
cargo run --release -- --vte-display-mode temporal-spacetime \
  --semantics temporal

# Show time direction as glyphs
cargo run --release -- --vte-display-mode temporal-arrow

# Highlight event voxels along the temporal axis
cargo run --release -- --vte-display-mode temporal-event
```

### Running Headless for Screenshots

```bash
cargo run --release -- --gpu-screenshot \
  --screenshot-pos 0 0 0 0 \
  --screenshot-angles-deg 0 0 0 0 \
  --backend voxel-traversal
```

## Troubleshooting

| Problem | Likely Cause | Solution |
|---------|-------------|----------|
| `vkEnumerateInstanceVersion` error | Vulkan SDK not installed | Install Vulkan SDK for your platform |
| Shader compilation fails | Slang compiler not in `PATH` | Install slang compiler from GitHub releases |
| `cargo build` takes very long | First build compiles many crates | Normal; subsequent builds are faster |
| `--headless` produces empty/black image | Vulkan headless not supported | Try without `--headless` or check GPU |

## Next Steps

- [ARCHITECTURE.md](./ARCHITECTURE.md) — Understand the rendering pipeline and system design
- [API_REFERENCE.md](./API_REFERENCE.md) — Public types, traits, and plugin API
- [LOW_LEVEL.md](./LOW_LEVEL.md) — Internal details and porting guide
- [docs/wasm-plugin-system.md](./docs/wasm-plugin-system.md) — Writing content plugins
