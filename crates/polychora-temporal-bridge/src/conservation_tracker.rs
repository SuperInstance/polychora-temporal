/// Tracks conservation properties (budget, profile, detect, report) along the W-axis.
pub struct ConservationTracker {
    /// Current conservation budget.
    budget: f64,
    /// Conservation profile value.
    profile: f64,
    /// Accumulated discrepancy detection count.
    detect_count: u64,
    /// Total time tracked.
    total_time: f64,
    /// Threshold ratio for triggering a conservation event.
    threshold: f64,
}

/// Describes a conservation event triggered when budget exceeds profile beyond
/// the configured threshold.
#[derive(Clone, Debug)]
pub struct ConservationEvent {
    /// The type of event.
    pub event_type: ConservationEventType,
    /// The budget value that triggered the event.
    pub budget: f64,
    /// The profile value at the time.
    pub profile: f64,
    /// The accumulated detection count.
    pub detect_count: u64,
}

/// Categories of conservation events.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConservationEventType {
    /// Budget exceeded profile beyond the threshold.
    BudgetExceeded,
    /// Budget fell back within acceptable range.
    BudgetRecovered,
    /// Profile threshold was reached or exceeded.
    ProfileBreach,
    /// Resource budget has been fully consumed.
    BudgetDepleted,
}

impl ConservationTracker {
    pub fn new() -> Self {
        Self {
            budget: 0.0,
            profile: 0.0,
            detect_count: 0,
            total_time: 0.0,
            threshold: 1.0,
        }
    }

    /// Set the conservation threshold ratio.
    pub fn set_threshold(&mut self, threshold: f64) {
        self.threshold = threshold;
    }

    /// Allocate additional budget.
    pub fn budget_allocation(&mut self, amount: f64) {
        self.budget += amount;
    }

    /// Set the conservation profile value.
    pub fn set_profile(&mut self, profile: f64) {
        self.profile = profile;
    }

    /// Query current budget.
    pub fn budget(&self) -> f64 {
        self.budget
    }

    /// Query current profile.
    pub fn profile(&self) -> f64 {
        self.profile
    }

    pub fn reset(&mut self) {
        self.budget = 0.0;
        self.profile = 0.0;
        self.detect_count = 0;
        self.total_time = 0.0;
    }

    /// Advance the internal clock, check resource budgets against profiles,
    /// and return any triggered conservation events.
    ///
    /// A conservation event is triggered when the budget exceeds the profile
    /// beyond the configured threshold ratio.
    pub fn tick(&mut self, dt: f64) -> Option<ConservationEvent> {
        self.total_time += dt;

        // Simulate budget consumption proportional to time step
        if dt > 0.0 {
            // Budget decays slightly per tick to model resource consumption
            self.budget = (self.budget - dt * 0.01).max(0.0);
        }

        // Check if budget exceeds profile beyond threshold
        if self.profile > 0.0 && self.budget > self.profile * self.threshold {
            self.detect_count += 1;
            return Some(ConservationEvent {
                event_type: ConservationEventType::BudgetExceeded,
                budget: self.budget,
                profile: self.profile,
                detect_count: self.detect_count,
            });
        }

        // Check if budget is fully depleted
        if self.budget <= 0.0 && self.profile > 0.0 && self.detect_count > 0 {
            return Some(ConservationEvent {
                event_type: ConservationEventType::BudgetDepleted,
                budget: self.budget,
                profile: self.profile,
                detect_count: self.detect_count,
            });
        }

        None
    }

    pub fn report(&self) -> ConservationReport {
        ConservationReport {
            budget: self.budget,
            profile: self.profile,
            detect_count: self.detect_count,
            total_time: self.total_time,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ConservationReport {
    pub budget: f64,
    pub profile: f64,
    pub detect_count: u64,
    pub total_time: f64,
}

impl Default for ConservationTracker {
    fn default() -> Self {
        Self::new()
    }
}
