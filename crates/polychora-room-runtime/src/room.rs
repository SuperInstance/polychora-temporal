use std::collections::HashMap;
use crate::tile::Tile;

/// A Room is a scoped gameplay context within the temporal 4D world.
/// It contains tiles, agents, a frozen context snapshot, and the creator's commit.
#[derive(Clone)]
pub struct Room {
    /// Unique scope identifier for this room.
    pub scope: String,
    /// Tiles within this room.
    pub tiles: Vec<Tile>,
    /// Active agent IDs within this room.
    pub agents: Vec<String>,
    /// A frozen context snapshot (serialized state).
    pub frozen_context: Vec<u8>,
    /// Creator's commit identifier for provenance.
    pub creator_commit: String,
    /// Additional metadata key-value store.
    pub metadata: HashMap<String, String>,
}

impl Room {
    pub fn new(scope: &str) -> Self {
        Self {
            scope: scope.to_string(),
            tiles: Vec::new(),
            agents: Vec::new(),
            frozen_context: Vec::new(),
            creator_commit: String::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn add_tile(&mut self, tile: Tile) {
        self.tiles.push(tile);
    }

    pub fn add_agent(&mut self, agent_id: &str) {
        if !self.agents.contains(&agent_id.to_string()) {
            self.agents.push(agent_id.to_string());
        }
    }

    /// Freeze the current context for stability during temporal operations.
    pub fn freeze_context(&mut self, context: Vec<u8>) {
        self.frozen_context = context;
    }

    pub fn is_frozen(&self) -> bool {
        !self.frozen_context.is_empty()
    }
}

impl Default for Room {
    fn default() -> Self {
        Self::new("default")
    }
}
