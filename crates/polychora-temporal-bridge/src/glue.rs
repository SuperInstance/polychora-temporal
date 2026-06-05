use crate::temporal_world::TemporalWorld;
use crate::conservation_tracker::ConservationTracker;

/// Auto-wiring infrastructure: detects Semantics4D::Temporal and
/// connects temporal crates.
pub struct TemporalGlue {
    /// Whether temporal mode is active.
    pub active: bool,
    /// Tracks conservation properties along W-axis.
    pub conservation: ConservationTracker,
}

impl TemporalGlue {
    pub fn new() -> Self {
        Self {
            active: false,
            conservation: ConservationTracker::new(),
        }
    }

    /// Initialize glue given the world's semantics.
    pub fn init(&mut self, world: &TemporalWorld) {
        if world.is_temporal() {
            self.active = true;
            self.conservation.reset();
            log::info!("Temporal glue initialized: temporal W-axis active");
        }
    }

    /// Run per-tick glue work: update conservation tracking, etc.
    pub fn tick(&mut self, _world: &TemporalWorld, dt: f64) {
        if !self.active {
            return;
        }
        self.conservation.tick(dt);
    }
}

impl Default for TemporalGlue {
    fn default() -> Self {
        Self::new()
    }
}
