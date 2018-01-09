
use chrono::{DateTime, Utc};

/// The result of an individual ping
#[derive(Debug, Copy, Clone)]
pub enum PingStatus {
    /// The ping was returned. The number of milliseconds taken is included.
    Returned(f64),
    /// The ping timed out
    Timeout,
}

/// The result of a ping
#[derive(Debug, Clone)]
pub struct PingResult {
    /// The time when the ping was recorded
    time: DateTime<Utc>,
    /// The ping status
    status: PingStatus,
}

impl PingResult {
    pub fn new(time: DateTime<Utc>, status: PingStatus) -> Self {
        PingResult {
            time,
            status,
        }
    }

    pub fn time(&self) -> DateTime<Utc> {
        self.time
    }
    pub fn status(&self) -> PingStatus {
        self.status
    }
}
