use std::time::{SystemTime, UNIX_EPOCH};

/// Provenance tracks the origin and modification history of tiles and rooms.
pub struct Provenance {
    /// Creator identity string.
    pub creator: String,
    /// Timestamp of creation (seconds since epoch).
    pub created_at: u64,
    /// Modification history entries.
    pub history: Vec<ProvenanceEntry>,
}

/// A single provenance entry recording an action and timestamp.
#[derive(Clone, Debug)]
pub struct ProvenanceEntry {
    pub action: String,
    pub timestamp: u64,
    pub actor: String,
}

impl Provenance {
    pub fn new(creator: &str) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Self {
            creator: creator.to_string(),
            created_at: now,
            history: Vec::new(),
        }
    }

    pub fn record(&mut self, action: &str, actor: &str) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        self.history.push(ProvenanceEntry {
            action: action.to_string(),
            timestamp: now,
            actor: actor.to_string(),
        });
    }

    pub fn is_empty(&self) -> bool {
        self.history.is_empty() && self.creator.is_empty()
    }
}

impl Default for Provenance {
    fn default() -> Self {
        Self::new("unknown")
    }
}
