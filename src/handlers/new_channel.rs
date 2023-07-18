use std::sync::Arc;

use actix_web::{web::{Json, Data}, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

use crate::{AppState, models::{channel::Channel, broadcast::Broadcaster}};


#[derive(Deserialize)]
pub struct NewChannelData {
    pub channel_name: String,
}


pub async fn new_channel(
    req: Json<Option<NewChannelData>>,
    state: Data<AppState>
) -> HttpResponse {
    let broadcaster = Broadcaster::create();


    if let Some(data) = req.into_inner() {
        let mut channels = state.channels.lock().unwrap(); 
        channels.push(
            Channel {
                id: Uuid::new_v4(),
                name: data.channel_name,
                users: Vec::new(),
                broadcaster: Arc::clone(&broadcaster),
            }
        );
        return HttpResponse::Ok().finish()
    }
    HttpResponse::BadRequest().finish()
}