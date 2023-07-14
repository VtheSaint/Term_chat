use actix_web::{web::Data, HttpResponse};

use crate::AppState;


pub async fn show_users(
    state: Data<AppState>
) -> HttpResponse {
    HttpResponse::Ok().json(state.users.lock().unwrap().clone())
}