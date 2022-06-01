use std::net::TcpListener;

fn spawn_app() -> String {
    let listner = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listner.local_addr().unwrap().port();
    let server = peppercorn::run(listner).expect("Failed to bind port");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.01:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_data(){
    let address = spawn_app();
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await.expect("Fialed");
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_400_for_invalid_data(){
    let address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le$20guin", "missing email"),
        ("email=ursula_le_guin%40gmail.com", "missing name"),
        ("", "missing both")
    ];
    for (invalid_body, error_message) in test_cases{
       let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Contnet-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await.expect("Fialed");
        assert_eq!(400, response.status().as_u16(), "Api did not fail when {}", error_message);
    }
}