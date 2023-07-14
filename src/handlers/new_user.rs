use actix_web::{web::{Json, Data}, HttpResponse, Responder};
use actix_web_lab::sse;
use serde::Deserialize;
use uuid::Uuid;

use crate::{AppState, models::{user::User, channel::Channel}};

#[derive(Deserialize)]
pub struct NewUser {
    pub name: String,
    pub channel_name: String,
}

pub async fn new_user(
    req: Json<Option<NewUser>>,
    state: Data<AppState>
) -> impl Responder {
    let mut response = sse::channel(10).1;
    // get data from request
    let data = req.into_inner().unwrap();

    // get channels from state
    let mut channels = state.channels.lock().unwrap();

    // iterate over channels and push new user
    for channel in channels.iter_mut() {
        if channel.name == data.channel_name {
            let new_user = User {
                id: Uuid::new_v4(),
                name: data.name.clone(),
            };
            // println!("users: {:?}", channel.users);
            // println!("state: {:?}", state.channels);
            response = Channel::add_user(channel, &new_user).await;
            

            // Construct message 
            let result = User::enter_channel(new_user, channel.name.as_str());
            
            // Publsih message to channel
            Channel::message(&channel, result).await;
        }

    }
    response
}