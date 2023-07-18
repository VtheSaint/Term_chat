use actix_web::web::{self, ServiceConfig};

use crate::handlers::{new_channel::new_channel, new_user::new_user, drop_user::drop_user, new_message::new_message, show_channels::show_channels, show_users::show_users};

pub fn routes_factory(app: &mut ServiceConfig) {
    app.service(
    
        web::scope("/api/v1")
        .service(
            web::scope("/create")
                .route("/channel", web::post().to(new_channel))
                .route("/user", web::post().to(new_user))
                .route("/message", web::post().to(new_message))
        )
            .route("/drop/user", web::post().to(drop_user))
            .route("/channels", web::get().to(show_channels))
            .route("/users", web::get().to(show_users))
        );
}

