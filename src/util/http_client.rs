use actix_web::client::{Client, Connector};
use openssl::ssl::{SslConnector, SslMethod};

pub fn create_http_client(token: String) -> Client {
    let builder = SslConnector::builder(SslMethod::tls()).unwrap();
    let client = Client::builder()
        .header("User-Agent", "localhost")
        .header("Authorization", format!("Bearer {}", token))
        .connector(Connector::new().ssl(builder.build()).finish())
        .finish();

    return client;
}
