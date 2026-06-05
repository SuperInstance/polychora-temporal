use common::semantics::Semantics4D;

/// A TemporalWorld composes temporal state with a tide clock
/// and semantics flag, enabling temporal W-axis interpretation.
pub struct TemporalWorld {
    /// Semantics flag: Spatial (original) or Temporal (refactored).
    pub semantics: Semantics4D,
    /// TideClock tracks periodic time ticks along the W-axis.
    pub tide_clock: TideClock,
    /// Current world tick index.
    pub tick: u64,
}

/// A simple tide clock that tracks time ticks and periods.
#[derive(Clone, Debug)]
pub struct TideClock {
    /// Current time tick along the W-axis.
    pub tick: u64,
    /// Tick rate (ticks per second).
    pub tick_rate: f64,
    /// Total elapsed time in seconds.
    pub elapsed: f64,
}

impl TideClock {
    pub fn new(tick_rate: f64) -> Self {
        Self {
            tick: 0,
            tick_rate,
            elapsed: 0.0,
        }
    }

    pub fn advance(&mut self, dt: f64) {
        self.elapsed += dt;
        self.tick = (self.elapsed * self.tick_rate) as u64;
    }

    pub fn reset(&mut self) {
        self.tick = 0;
        self.elapsed = 0.0;
    }
}

impl TemporalWorld {
    pub fn new(tick_rate: f64) -> Self {
        Self {
            tide_clock: TideClock::new(tick_rate),
            semantics: Semantics4D::Spatial,
            tick: 0,
        }
    }

    pub fn with_semantics(mut self, semantics: Semantics4D) -> Self {
        self.semantics = semantics;
        self
    }

    pub fn is_temporal(&self) -> bool {
        self.semantics == Semantics4D::Temporal
    }

    pub fn advance_time(&mut self, dt: f64) {
        if self.is_temporal() {
            self.tide_clock.advance(dt);
            self.tick = self.tide_clock.tick;
        }
    }
}
