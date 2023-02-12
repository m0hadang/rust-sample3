use std::net::TcpListener;

fn spawn_app() -> String { // Test용 앱 실행
  let listener = TcpListener::bind("127.0.0.1:8888").expect("Failed to bind port");
  let port = listener.local_addr().unwrap().port();
  let server = zero2prod::run(listener).expect("Failed to bind address");
  let _ = tokio::spawn(server);

  format!("http://127.0.0.1:{}", port)
}


// GET api : /subscriptions
#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new(); // http web request 라이브러리

    let response = client
        .get(&format!("{}/health_check", &address)) // GET 요청
        .send() // SEND
        .await //대기
        .expect("Failed to execute request."); // 에러 처리

    assert!(response.status().is_success()); // 
    assert_eq!(Some(0), response.content_length());
}

// POST api : /subscriptions
#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=kane%20-3000&email=dev%40test.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=kane%-3000", "missing the email"),
        ("email=dev%40test.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}