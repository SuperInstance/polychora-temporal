/// Determines whether the 4th axis (W) is spatial or temporal.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum Semantics4D {
    #[default]
    Spatial,  // W = 4th spatial axis (polychora original)
    Temporal, // W = time (our refactor)
}
