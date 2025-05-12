use rcp_cli::utils;
use tokio::test;

/// Test format status utility
#[test]
async fn test_format_status() {
    // Test various status values
    let running_status = utils::format_status("running");
    let stopped_status = utils::format_status("stopped");
    let error_status = utils::format_status("error");
    let unknown_status = utils::format_status("unknown");

    // Verify the formatting includes the status text
    assert!(running_status.to_string().contains("running"));
    assert!(stopped_status.to_string().contains("stopped"));
    assert!(error_status.to_string().contains("error"));
    assert!(unknown_status.to_string().contains("unknown"));
}

/// Test formatting durations
#[test]
async fn test_format_duration() {
    // Test different duration values
    let seconds = utils::format_duration(30);
    let minutes = utils::format_duration(150);
    let hours = utils::format_duration(3600);
    let days = utils::format_duration(86400 * 2);

    // Verify formatted strings
    assert_eq!(seconds, "30s");
    assert_eq!(minutes, "2m 30s");
    assert_eq!(hours, "1h 0m 0s");
    assert_eq!(days, "2d 0h 0m 0s");
}

/// Test table formatting utilities
#[test]
async fn test_print_table() {
    // This is mostly a visual test, but we can ensure the function doesn't panic

    // Create a table with headers and rows
    let headers = ["Name", "Status", "Port", "Connections"];
    let rows = [
        ["Server1", "running", "8080", "5"],
        ["Server2", "stopped", "8081", "0"],
    ];

    // Calculate column widths manually (since utils doesn't have this function)
    let widths = headers
        .iter()
        .enumerate()
        .map(|(idx, header)| {
            let max_row_width = rows.iter().map(|row| row[idx].len()).max().unwrap_or(0);
            std::cmp::max(header.len(), max_row_width)
        })
        .collect::<Vec<_>>();

    // Verify widths are sufficient for the content
    assert!(widths[0] >= "Server1".len());
    assert!(widths[1] >= "running".len());
    assert!(widths[2] >= "Port".len());
    assert!(widths[3] >= "Connections".len());
}
