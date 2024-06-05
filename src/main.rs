use std::net::TcpListener;

use actix_email_newsletter::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("http://127.0.0.1:8000").expect("Failed to bind a listener");
    run(listener)?.await
}
