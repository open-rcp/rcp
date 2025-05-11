use rcp_api::api;
use serde_json::json;

/// Test API response serialization
#[test]
fn test_api_response_success() {
    // Create a success response
    let data = json!({
        "id": "123",
        "name": "Test"
    });
    
    let response = api::ApiResponse::success(data);
    
    // Serialize to JSON
    let json = serde_json::to_value(response).expect("Failed to serialize response");
    
    // Verify response structure
    assert_eq!(json["success"], true);
    assert_eq!(json["data"]["id"], "123");
    assert_eq!(json["data"]["name"], "Test");
    assert!(json.get("error").is_none());
}

/// Test API response with error
#[test]
fn test_api_response_error() {
    // Create an error response
    let response = api::ApiResponse::<()>::error("Not Found");
    
    // Serialize to JSON
    let json = serde_json::to_value(response).expect("Failed to serialize response");
    
    // Verify response structure
    assert_eq!(json["success"], false);
    assert_eq!(json["error"]["message"], "Not Found");
    assert_eq!(json["error"]["code"], 404);
    assert!(json.get("data").is_none());
}

/// Test API pagination response
#[test]
fn test_api_pagination_response() {
    // Create sample data
    let items = vec![
        json!({"id": "1", "name": "Item 1"}),
        json!({"id": "2", "name": "Item 2"}),
        json!({"id": "3", "name": "Item 3"}),
    ];
    
    // Create paginated response
    let response = api::PaginatedResponse::new(
        items,
        2,  // page
        3,  // page_size
        10, // total
    );
    
    // Serialize to JSON
    let json = serde_json::to_value(response).expect("Failed to serialize response");
    
    // Verify response structure
    assert_eq!(json["items"].as_array().unwrap().len(), 3);
    assert_eq!(json["page"], 2);
    assert_eq!(json["page_size"], 3);
    assert_eq!(json["total"], 10);
}
