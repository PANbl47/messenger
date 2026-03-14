#[tokio::test]
async fn health_endpoint_returns_ok() {
    let response = messenger_gateway::test_support::health().await;
    assert_eq!(response.status(), 200);
}
