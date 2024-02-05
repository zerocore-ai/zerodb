use std::time::{Duration, Instant};

use rand::Rng;
use tokio::time::{self, Sleep};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// `Countdown` is a utility struct for countdowns that you can get a continuation of.
///
/// It provides functionality to start a countdown, get a continuation countdown, reset the countdown,
/// and retrieve the elapsed time and the current interval.
///
/// This struct is particularly useful in leader election algorithms where nodes need to wait for
/// a random period before starting an election.
#[derive(Debug, Clone)]
pub struct Countdown {
    /// The amount of time to wait before the countdown is complete.
    duration: PossibleDuration,
    /// The time at which the countdown started.
    start: Instant,
    /// The current countdown interval.
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

impl Countdown {
    /// Starts a new countdown with a fixed duration.
    pub fn start(duration: u64) -> Self {
        let current_interval = Duration::from_millis(duration);
        Self {
            duration: PossibleDuration::Fixed(duration),
            start: Instant::now(),
            current_interval,
        }
    }

    /// Starts a new countdown with a random duration within a range.
    pub fn start_range((min, max): (u64, u64)) -> Self {
        let current_interval = rand::thread_rng().gen_range(min..max);
        Self {
            duration: PossibleDuration::Range(min, max),
            start: Instant::now(),
            current_interval: Duration::from_millis(current_interval),
        }
    }

    /// Returns a continuation of the countdown.
    pub fn continuation(&self) -> Sleep {
        let continuation = self.current_interval - self.start.elapsed();
        time::sleep(continuation)
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
    async fn test_fixed_countdown_elapsed_correctly() {
        let countdown = Countdown::start(200);
        time::sleep(countdown.get_interval()).await;
        assert!(countdown.get_elapsed() >= countdown.get_interval());
    }

    #[tokio::test]
    async fn test_range_countdown_elapsed_correctly() {
        let countdown = Countdown::start_range((200, 400));
        time::sleep(countdown.get_interval()).await;
        assert!(countdown.get_elapsed() >= countdown.get_interval());
    }

    #[tokio::test]
    async fn test_fixed_continuation_elapses_with_countdown() {
        let countdown = Countdown::start(200);
        time::sleep(countdown.get_interval() / 2).await;
        countdown.continuation().await;
        assert!(countdown.get_elapsed() >= countdown.get_interval());
    }

    #[tokio::test]
    async fn test_range_continuation_elapses_with_countdown() {
        let countdown = Countdown::start_range((200, 400));
        time::sleep(countdown.get_interval() / 2).await;
        countdown.continuation().await;
        assert!(countdown.get_elapsed() >= countdown.get_interval());
    }

    #[tokio::test]
    async fn test_fixed_countdown_can_be_reset() {
        let mut countdown = Countdown::start(200);
        time::sleep(countdown.get_interval() / 2).await;

        countdown.reset();

        time::sleep(countdown.get_interval()).await;
        assert!(countdown.get_elapsed() >= countdown.get_interval());
    }

    #[tokio::test]
    async fn test_range_countdown_can_be_reset() {
        let mut countdown = Countdown::start_range((200, 400));
        time::sleep(countdown.get_interval() / 2).await;

        countdown.reset();

        time::sleep(countdown.get_interval()).await;
        assert!(countdown.get_elapsed() >= countdown.get_interval());
    }
}
