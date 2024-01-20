use crate::Result;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

// TODO(appcypher): Implement this
pub struct RaftRequest {}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl RaftRequest {
    // TODO(appcypher): Implement this
    pub(crate) fn from_cbor(_buf: Vec<u8>) -> Result<Self> {
        Ok(Self {})
    }

    // TODO(appcypher): Implement this
    pub(crate) fn to_cbor(&self) -> Result<Vec<u8>> {
        Ok(vec![])
    }
}
