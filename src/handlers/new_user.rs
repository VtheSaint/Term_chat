use actix_web::{web::{Json, Data}, HttpResponse};
use uuid::Uuid;

use crate::{AppState, models::{user::User, channel::Channel}};


pub struct NewUser {
    pub name: String,
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

    // iterate over channels and push new user
    for channel in channels.iter_mut() {
        if channel.id == data.channel_id {
            let new_user = User {
                id: Uuid::new_v4(),
                name: data.name.clone(),
            };
            Channel::add_user(channel, &new_user);
            
            // Construct message 
            let result = User::enter_channel(new_user, channel.name.as_str());
            
            // Publsih message to channel
            Channel::message(&channel, result);
            break;
        }
    }
    HttpResponse::Ok().finish()
}