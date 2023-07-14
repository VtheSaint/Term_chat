use actix_web::{web::{self, ServiceConfig, Json, post}, Responder, HttpResponse, post, get};

use crate::{models::user::User, handlers::{new_channel::new_channel, new_user::new_user, drop_user::drop_user, new_message::new_message, show_channels::show_channels, show_users::show_users}};

pub fn routes_factory(app: &mut ServiceConfig) {
    app.service(
        
        web::scope("/api/v1")
        .service(
            web::scope("/create")
                .route("channel", web::post().to(new_channel))
                .route("user", web::post().to(new_user))
                .route("message", web::post().to(new_message))
        )
            // .route("drop/channel", post().to(drop_channel))
            .route("drop/user", web::post().to(drop_user))
            .route("channels", web::get().to(show_channels))
            .route("users", web::get().to(show_users))
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

