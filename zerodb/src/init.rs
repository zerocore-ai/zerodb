use crate::ZerodbResult;

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// This is where general initialization code goes. For example, logging gets initialized here.
pub fn init() -> ZerodbResult<()> {
    tracing_subscriber::fmt::init();
    Ok(())
}
