use std::time::Duration;

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

pub(crate) async fn election(timeout: Duration) {
    tokio::time::sleep(timeout).await;
}

pub(crate) async fn heartbeat(timeout: Duration) {
    tokio::time::sleep(timeout).await;
}
