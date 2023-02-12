use zero2prod::run;

use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // TCP 리스너 소켓 생성하여 bind 까지 처리
    let listener = TcpListener::bind("127.0.0.1:8888").expect("Failed to bind port");
    run(listener)?.await
}