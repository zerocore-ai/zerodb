//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

/// A trait capturing and restoring the state of an entity.
pub trait Reversible {
    /// The type of the state.
    type State;

    /// Get the state of the entity.
    fn get_state(&self) -> Self::State;

    /// Set the state of the entity.
    fn set_state(&mut self, state: Self::State);
}
