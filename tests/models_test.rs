/*
 * Tests for model serialization and deserialization
 */

use agno_agentos_client::models::HealthResponse;

#[test]
fn test_health_response_serialization() {
    // Create a HealthResponse instance
    let health_response = HealthResponse::new("healthy".to_string(), "1234567890".to_string());

    // Test serialization
    let json = serde_json::to_string(&health_response);
    assert!(json.is_ok(), "HealthResponse should serialize successfully");

    let json_str = json.unwrap();
    assert!(
        json_str.contains("\"status\":\"healthy\""),
        "Serialized JSON should contain status field"
    );
    assert!(
        json_str.contains("\"instantiated_at\":\"1234567890\""),
        "Serialized JSON should contain instantiated_at field"
    );
}

#[test]
fn test_health_response_deserialization() {
    // Test JSON string
    let json_str = r#"{"status":"healthy","instantiated_at":"1234567890"}"#;

    // Test deserialization
    let result: Result<HealthResponse, _> = serde_json::from_str(json_str);
    assert!(
        result.is_ok(),
        "HealthResponse should deserialize successfully"
    );

    let health_response = result.unwrap();
    assert_eq!(
        health_response.status, "healthy",
        "Deserialized status should match"
    );
    assert_eq!(
        health_response.instantiated_at, "1234567890",
        "Deserialized instantiated_at should match"
    );
}

#[test]
fn test_health_response_round_trip() {
    // Create a HealthResponse
    let original = HealthResponse::new("healthy".to_string(), "1234567890".to_string());

    // Serialize to JSON
    let json_str = serde_json::to_string(&original).unwrap();

    // Deserialize back
    let deserialized: HealthResponse = serde_json::from_str(&json_str).unwrap();

    // Verify round trip
    assert_eq!(
        original.status, deserialized.status,
        "Status should match after round trip"
    );
    assert_eq!(
        original.instantiated_at, deserialized.instantiated_at,
        "Instantiated_at should match after round trip"
    );
}

#[test]
fn test_health_response_new() {
    // Test the new() constructor
    let health_response = HealthResponse::new("unhealthy".to_string(), "9876543210".to_string());

    assert_eq!(health_response.status, "unhealthy");
    assert_eq!(health_response.instantiated_at, "9876543210");
}
