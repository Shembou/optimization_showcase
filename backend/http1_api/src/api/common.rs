use actix_web::{HttpResponse, Responder, get};

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("Service working properly")
}
