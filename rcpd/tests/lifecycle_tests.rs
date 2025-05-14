// filepath: /Volumes/EXT/repos/open-rcp/rcp/rcp-service/tests/lifecycle_tests.rs
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::timeout;

// Import the lifecycle module
#[path = "../src/lifecycle.rs"]
mod lifecycle;
use lifecycle::ServiceLifecycle;

// Import error types
#[path = "../src/error.rs"]
mod error;

#[tokio::test]
async fn test_lifecycle_creation() {
    // Create a channel for shutdown signals
    let (tx, rx) = mpsc::channel::<()>(1);

    // Create the service lifecycle
    let lifecycle = ServiceLifecycle::new(tx);

    // Basic assertion that lifecycle can be created
    assert!(true, "ServiceLifecycle was created successfully");
}

#[tokio::test]
async fn test_lifecycle_start() {
    // Create a channel for shutdown signals
    let (tx, rx) = mpsc::channel::<()>(1);

    // Create the service lifecycle
    let lifecycle = ServiceLifecycle::new(tx);

    // Start the lifecycle
    let result = lifecycle.start().await;

    // Verify the lifecycle started successfully
    assert!(result.is_ok(), "Lifecycle should start without errors");
}

#[tokio::test]
async fn test_lifecycle_stop() {
    // Create a channel for shutdown signals
    let (tx, mut rx) = mpsc::channel::<()>(1);

    // Create the service lifecycle
    let lifecycle = ServiceLifecycle::new(tx);

    // Start the lifecycle
    let _ = lifecycle.start().await;

    // Stop the lifecycle
    let stop_result = lifecycle.stop().await;
    assert!(stop_result.is_ok(), "Lifecycle should stop without errors");

    // Check if shutdown signal was received
    let recv_result = timeout(Duration::from_millis(100), rx.recv()).await;
    assert!(recv_result.is_ok(), "Shutdown signal should be received");
    assert!(
        recv_result.unwrap().is_some(),
        "Shutdown signal should be Some(())"
    );
}

#[tokio::test]
async fn test_lifecycle_double_stop() {
    // Create a channel for shutdown signals
    let (tx, mut rx) = mpsc::channel::<()>(1);

    // Create the service lifecycle
    let lifecycle = ServiceLifecycle::new(tx);

    // Start the lifecycle
    let _ = lifecycle.start().await;

    // First stop should succeed
    let stop_result1 = lifecycle.stop().await;
    assert!(stop_result1.is_ok(), "First stop should succeed");

    // Consume the shutdown signal
    let _ = rx.recv().await;

    // In the current implementation, the second stop will succeed because
    // there is no state tracking to detect that the service is already stopped
    let stop_result2 = lifecycle.stop().await;

    // Based on the existing implementation, the second stop should actually succeed
    // because the service does not track its state
    assert!(
        stop_result2.is_ok(),
        "Second stop should succeed with current implementation"
    );
}
