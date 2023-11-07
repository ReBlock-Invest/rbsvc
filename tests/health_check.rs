fn spawn_app() {
    let server = rbsvc::run().expect("Failed to bind address");

    let _ = tokio::spawn(server);
}

#[tokio::test]
async fn health_check_works() {
    spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8000/health")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn issue_returns_a_200_for_valid_data() {
    spawn_app();
    let client = reqwest::Client::new();

    let body = "{\"id\":\"1\",\"recipient\":\"0xAA\"}";
    let response = client
        .post(&format!("http://127.0.0.1:8000/issue"))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}
