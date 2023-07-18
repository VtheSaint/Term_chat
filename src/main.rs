use std::{env, sync::{Mutex, Arc}};

use actix_web::{HttpServer, App, middleware::Logger, web::Data};
use dotenvy::dotenv;
use models::{channel::Channel, user::User};

pub mod routes;
pub mod models;
pub mod handlers;

#[derive(Clone)]
pub struct AppState {
    pub channels: Arc<Mutex<Vec<Channel>>>,
    pub users: Arc<Mutex<Vec<User>>>
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Initialize state
    let state = AppState {
        channels: Arc::new(Mutex::new(Vec::new())),
        users: Arc::new(Mutex::new(Vec::new()))
    };

    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();
    

    let server_url = env::var("SERVER_URL")
        .expect("SERVER_URL must be set");


    HttpServer::new(move || {
        App::new()
        .app_data(Data::new(state.clone()))
        .wrap(Logger::default())
        .configure(routes::routes_factory)
    })
    .bind(server_url)?
    .run()
    .await
}