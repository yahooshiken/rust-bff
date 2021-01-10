use actix_web::{
    client::{Client, Connector},
    get, post, web, App, HttpResponse, HttpServer, Responder,
};
use openssl::ssl::{SslConnector, SslMethod};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World.")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/v1/activities/github")]
async fn get_activities_from_github() -> impl Responder {
    let builder = SslConnector::builder(SslMethod::tls()).unwrap();
    let client = Client::builder()
        .header("User-Agent", "localhost")
        .connector(Connector::new().ssl(builder.build()).finish())
        .finish();
    let url = "https://api.github.com/zen";
    let payload = client.get(url).send().await;
    println!("Response: {:?}", payload);

    return HttpResponse::Ok().body("Hello World.");
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey, there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(get_activities_from_github)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
