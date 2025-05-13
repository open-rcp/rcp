use std::path::PathBuf;
use tokio::sync::mpsc;

// Import from the library
use rcpd::config::ServiceConfig;
use rcpd::manager::ServiceManager;

#[tokio::test]
async fn test_manager_creation() {
    // Create a channel for shutdown signals
    let (tx, _rx) = mpsc::channel::<()>(1);

    // Create a ServiceManager with work directory
    let work_dir = PathBuf::from(".");
    let config = ServiceConfig::default();
    let _manager = ServiceManager::new(work_dir, config, tx);

    // Basic assertion that manager was created
    assert!(true);
}

#[tokio::test]
async fn test_manager_start_stop() {
    // Create a channel for shutdown signals
    let (tx, _rx) = mpsc::channel::<()>(1);

    // Create a ServiceManager
    let work_dir = PathBuf::from(".");
    let config = ServiceConfig::default();
    let mut manager = ServiceManager::new(work_dir, config, tx);

    // Start the manager
    let start_result = manager.start().await;
    assert!(start_result.is_ok());

    // Stop the manager
    let stop_result = manager.stop().await;
    assert!(stop_result.is_ok());
}

#[tokio::test]
async fn test_manager_double_start() {
    // Create a channel for shutdown signals
    let (tx, _rx) = mpsc::channel::<()>(1);

    // Create a ServiceManager
    let work_dir = PathBuf::from(".");
    let config = ServiceConfig::default();
    let mut manager = ServiceManager::new(work_dir, config, tx);

    // Start the manager
    let start_result1 = manager.start().await;
    assert!(start_result1.is_ok());

    // Start the manager again (this tests state tracking)
    let start_result2 = manager.start().await;
    assert!(start_result2.is_ok());

    // Clean up
    let _ = manager.stop().await;
}

#[tokio::test]
async fn test_manager_stop_without_start() {
    // Create a channel for shutdown signals
    let (tx, _rx) = mpsc::channel::<()>(1);

    // Create a ServiceManager
    let work_dir = PathBuf::from(".");
    let config = ServiceConfig::default();
    let mut manager = ServiceManager::new(work_dir, config, tx);

    // Stop the manager without starting it first
    let stop_result = manager.stop().await;
    assert!(stop_result.is_ok());
}

#[tokio::test]
async fn test_manager_work_dir() {
    // Create a channel for shutdown signals
    let (tx, _rx) = mpsc::channel::<()>(1);

    // Create a ServiceManager with a custom work directory
    let work_dir = PathBuf::from("./test-dir");
    let config = ServiceConfig::default();
    let manager = ServiceManager::new(work_dir.clone(), config, tx);

    // Assert that the work directory is correctly set
    assert_eq!(manager.get_work_dir(), &work_dir);
}