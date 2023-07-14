use std::{env, sync::Mutex};

use actix_web::{HttpServer, App, middleware::Logger, web::Data};
use dotenvy::dotenv;
use models::{channel::Channel, user::User};

pub mod routes;
pub mod models;
pub mod handlers;


pub struct AppState {
    pub channels: Mutex<Vec<Channel>>,
    pub users: Mutex<Vec<User>>
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();
    
    let server_url = env::var("SERVER_URL")
        .expect("SERVER_URL must be set");

    HttpServer::new(move || {
        App::new()
        //.wrap(CORS::default())
        .app_data(Data::new(
            AppState { 
                channels: Mutex::new(Vec::new()),
                users: Mutex::new(Vec::new())
             }
        ))
        .wrap(Logger::default())
        .configure(routes::routes_factory)
    })
    .bind(server_url)?
    .run()
    .await
}