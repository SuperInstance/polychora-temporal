/// A Tile defines a bounded region with presets, a creator context hash,
/// required agents, and capabilities for temporal gameplay.
#[derive(Clone)]
pub struct Tile {
    /// Preset data for this tile (visual/behavioral configuration).
    pub presets: Vec<String>,
    /// Hash of the creator's context for provenance verification.
    pub creator_context_hash: [u8; 32],
    /// Required agents that must be present for this tile to function.
    pub required_agents: Vec<String>,
    /// Capability flags for this tile.
    pub capabilities: TileCapabilities,
}

/// Capabilities a tile can support.
#[derive(Clone, Debug)]
pub struct TileCapabilities {
    /// Can host agents.
    pub host_agents: bool,
    /// Supports temporal operations.
    pub temporal: bool,
    /// Supports persistence across ticks.
    pub persistent: bool,
    /// Supports event emission.
    pub emit_events: bool,
}

impl Default for TileCapabilities {
    fn default() -> Self {
        Self {
            host_agents: true,
            temporal: false,
            persistent: true,
            emit_events: false,
        }
    }
}

impl Tile {
    pub fn new() -> Self {
        Self {
            presets: Vec::new(),
            creator_context_hash: [0u8; 32],
            required_agents: Vec::new(),
            capabilities: TileCapabilities::default(),
        }
    }

    pub fn with_capabilities(mut self, caps: TileCapabilities) -> Self {
        self.capabilities = caps;
        self
    }

    pub fn add_preset(&mut self, preset: &str) {
        self.presets.push(preset.to_string());
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self::new()
    }
}
