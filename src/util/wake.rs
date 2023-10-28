use actix_web::{get, HttpResponse, Responder};

#[get("/v1/wake")]
pub async fn wake_up_server() -> impl Responder {
  return HttpResponse::Ok();
}
