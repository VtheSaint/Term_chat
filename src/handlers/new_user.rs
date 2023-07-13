use actix_web::{web::{Json, Data}, HttpResponse};
use uuid::Uuid;

use crate::{AppState, models::{user::User, channel::Channel}};


pub struct NewUser {
    pub name: String,
    // TODO : Realise get_channels handler 
    pub channel_id: Uuid,
}

pub fn new_user(
    req: Json<Option<NewUser>>,
    state: Data<AppState>
) -> HttpResponse {

    // get data from request
    let data = req.into_inner().unwrap();

    // get channels from state
    let mut channels = state.channels.lock().unwrap();

    for channel in channels {
    }
    let current_user = User {
        id: Uuid::new_v4(),
        name: data.name.clone(),
    };

    Channel::add_user(&mut current_channel, current_user);
    // let result = User::enter_channel(self, group)

    HttpResponse::Ok().finish()
}