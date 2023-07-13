use actix_web::{web::{self, ServiceConfig, Json}, Responder, HttpResponse, get, post};

use crate::models::user::User;

pub fn routes_factory(app: &mut ServiceConfig) {
    app.service(
        
        web::scope("/api/v1")
        .service(index)
        );
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json("It's alive!")
}

#[post("/push")]
pub async fn push(
    req: Json<Option<User>>,
    user: web::Data<User>,
) -> impl Responder {
    HttpResponse::Ok().json("Added!")
}

