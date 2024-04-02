use std::time::{Duration, Instant};

use rand::Rng;
use tokio::time::{self, Sleep};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// `Timeout` is a utility struct for timeouts that you can get a continuation of.
///
/// It provides functionality to start a timeout, get a continuation timeout, reset the timeout,
/// and retrieve the elapsed time and the current interval.
///
/// This struct is particularly useful in leader election algorithms where nodes need to wait for
/// a random period before starting an election.
#[derive(Debug, Clone)]
pub struct Timeout {
    /// The amount of time to wait before the timeout is complete.
    duration: PossibleDuration,
    /// The time at which the timeout started.
    start: Instant,
    /// The current timeout interval.
    current_interval: Duration,
}

#[derive(Debug, Clone)]
enum PossibleDuration {
    Range(u64, u64),
    Fixed(u64),
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Timeout {
    /// Starts a new timeout with a fixed duration.
    pub fn start(duration: u64) -> Self {
        let current_interval = Duration::from_millis(duration);
        Self {
            duration: PossibleDuration::Fixed(duration),
            start: Instant::now(),
            current_interval,
        }
    }

    /// Starts a new timeout with a random duration within a range.
    pub fn start_range((min, max): (u64, u64)) -> Self {
        let current_interval = rand::thread_rng().gen_range(min..max);
        Self {
            duration: PossibleDuration::Range(min, max),
            start: Instant::now(),
            current_interval: Duration::from_millis(current_interval),
        }
    }

    /// Returns a continuation of the timeout.
    pub fn continuation(&self) -> Sleep {
        time::sleep(self.get_remaining())
    }

    /// Resets the election timeout.
    pub fn reset(&mut self) {
        *self = match self.duration {
            PossibleDuration::Range(min, max) => Self::start_range((min, max)),
            PossibleDuration::Fixed(duration) => Self::start(duration),
        };
    }

    /// Returns the elapsed time since the election timeout started.
    pub fn get_elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    /// Returns the remaining time until the timeout is complete.
    pub fn get_remaining(&self) -> Duration {
        self.current_interval - self.start.elapsed()
    }

    /// Gets the current election timeout interval.
    pub fn get_interval(&self) -> Duration {
        self.current_interval
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fixed_timeout_elapsed_correctly() {
        let timeout = Timeout::start(200);
        time::sleep(timeout.get_interval()).await;
        assert!(timeout.get_elapsed() >= timeout.get_interval());
    }

    #[tokio::test]
    async fn test_range_timeout_elapsed_correctly() {
        let timeout = Timeout::start_range((200, 400));
        time::sleep(timeout.get_interval()).await;
        assert!(timeout.get_elapsed() >= timeout.get_interval());
    }

    #[tokio::test]
    async fn test_fixed_continuation_elapses_with_timeout() {
        let timeout = Timeout::start(200);
        time::sleep(timeout.get_interval() / 2).await;
        timeout.continuation().await;
        assert!(timeout.get_elapsed() >= timeout.get_interval());
    }

    #[tokio::test]
    async fn test_range_continuation_elapses_with_timeout() {
        let timeout = Timeout::start_range((200, 400));
        time::sleep(timeout.get_interval() / 2).await;
        timeout.continuation().await;
        assert!(timeout.get_elapsed() >= timeout.get_interval());
    }

    #[tokio::test]
    async fn test_fixed_timeout_can_be_reset() {
        let mut timeout = Timeout::start(200);
        time::sleep(timeout.get_interval() / 2).await;

        timeout.reset();

        time::sleep(timeout.get_interval()).await;
        assert!(timeout.get_elapsed() >= timeout.get_interval());
    }

    #[tokio::test]
    async fn test_range_timeout_can_be_reset() {
        let mut timeout = Timeout::start_range((200, 400));
        time::sleep(timeout.get_interval() / 2).await;

        timeout.reset();

        time::sleep(timeout.get_interval()).await;
        assert!(timeout.get_elapsed() >= timeout.get_interval());
    }
}
