use actix_web::{web::Data, HttpResponse};
use serde::Serialize;

use crate::{AppState, models::user::User};

#[derive(Serialize)]
struct ShowData {
    channel_name: String,
    users: Vec<User>,
}

pub async fn show_channels(
    state: Data<AppState>,
) -> HttpResponse {
    let res = state.channels.lock().unwrap().clone();
    let mut result: Vec<ShowData> = Vec::new();
    for chn in res.iter() {

        
        // println!("{:?}", chn.name);


        result.push(
            ShowData {
                channel_name: chn.name.clone(),
                users: chn.users.clone(),
            },
        );
    }
    HttpResponse::Ok().json(result)
}