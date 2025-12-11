/*
 * Tests for API functions
 */

use agno_agentos_client::apis::configuration::Configuration;
use agno_agentos_client::apis::health_api::health_check_request_builder;

#[tokio::test]
async fn test_health_check_request_builder() {
    // Create a test configuration
    let config = Configuration {
        base_path: "https://api.example.com".to_string(),
        user_agent: Some("test-agent/1.0".to_string()),
        ..Default::default()
    };

    // Test that the request builder is created successfully
    let result = health_check_request_builder(&config);
    assert!(
        result.is_ok(),
        "Request builder should be created successfully"
    );

    // Verify the request builder can build a request
    let req_builder = result.unwrap();
    let req = req_builder.build();
    assert!(req.is_ok(), "Request should be built successfully");

    let request = req.unwrap();
    assert_eq!(
        request.url().to_string(),
        "https://api.example.com/health",
        "URL should match expected health endpoint"
    );
    assert_eq!(
        request.method(),
        reqwest::Method::GET,
        "HTTP method should be GET"
    );
}

#[tokio::test]
async fn test_health_check_request_builder_with_default_config() {
    // Test with default configuration
    let config = Configuration::default();
    let result = health_check_request_builder(&config);
    assert!(
        result.is_ok(),
        "Request builder should work with default config"
    );

    let req_builder = result.unwrap();
    let req = req_builder.build().unwrap();
    assert_eq!(
        req.url().to_string(),
        "http://localhost/health",
        "Default base path should be used"
    );
}
