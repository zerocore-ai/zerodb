//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

/// A trait capturing the state of a parser.
pub trait StateCapture {
    /// The type of the state.
    type State;

    /// Get the state of the parser.
    fn get_state(&self) -> Self::State;

    /// Set the state of the parser.
    fn set_state(&mut self, state: Self::State);
}
