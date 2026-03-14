use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryState {
    WaitingForNetwork,
    RetryNow,
    FailedRetryable,
}

#[derive(Debug, Clone, Copy)]
pub struct RetryPolicy {
    failure_timeout: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            failure_timeout: Duration::from_secs(180),
        }
    }
}

impl RetryPolicy {
    pub fn evaluate(&self, queued_for: Duration, network_available: bool) -> RetryState {
        if network_available {
            return RetryState::RetryNow;
        }

        if queued_for > self.failure_timeout {
            return RetryState::FailedRetryable;
        }

        RetryState::WaitingForNetwork
    }
}
