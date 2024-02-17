use awc::Client;

pub fn create_http_client(token: String) -> Client {
    let client = Client::builder()
        .add_default_header(("User-Agent", "localhost"))
        .add_default_header(("Authorization", format!("Bearer {}", token)))
        .finish();

    return client;
}
