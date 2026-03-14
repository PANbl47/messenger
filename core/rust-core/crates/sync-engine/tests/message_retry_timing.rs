use std::time::Duration;

use sync_engine::{RetryPolicy, RetryState};

#[test]
fn waits_for_network_before_timeout() {
    let policy = RetryPolicy::default();

    let state = policy.evaluate(Duration::from_secs(120), false);

    assert_eq!(state, RetryState::WaitingForNetwork);
}

#[test]
fn becomes_retryable_failure_after_three_minutes() {
    let policy = RetryPolicy::default();

    let state = policy.evaluate(Duration::from_secs(181), false);

    assert_eq!(state, RetryState::FailedRetryable);
}

#[test]
fn reconnecting_before_timeout_retries_immediately() {
    let policy = RetryPolicy::default();

    let state = policy.evaluate(Duration::from_secs(30), true);

    assert_eq!(state, RetryState::RetryNow);
}
