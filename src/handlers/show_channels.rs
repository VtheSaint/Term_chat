use actix_web::{web::Data, HttpResponse};

use crate::AppState;



pub async fn show_channels(
    state: Data<AppState>,
) -> HttpResponse {
    let res = state.channels.lock().unwrap().clone();
    let mut result: Vec<String> = Vec::new();
    for chn in res.iter() {
        println!("{:?}", chn.name);
        result.push(chn.clone().name);
    }
    HttpResponse::Ok().json(result)
}