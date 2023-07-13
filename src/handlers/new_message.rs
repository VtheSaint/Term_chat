use actix_web::{web::{Json, Data}, HttpResponse};
use uuid::Uuid;

use crate::{models::{user::User, channel::Channel}, AppState};

pub struct MessageData {
    pub user_id: Uuid,
    pub channel_id: Uuid,
    pub message: String,
}


pub fn new_message(
    req: Json<Option<MessageData>>,
    state: Data<AppState>
) -> HttpResponse {

    
    // Get data from request
    let data = req.into_inner().unwrap();
    
    // get users and channels from state
    let users = state.users.lock().unwrap();
    let channels = state.channels.lock().unwrap();

    // Get current user and channel from Vecs 
    let current_user = *(users.iter().filter(|u| u.id == data.user_id).collect::<Vec<&User>>().last().unwrap());
    let current_channel = *(channels.iter().filter(|c| c.id == data.channel_id).collect::<Vec<&Channel>>().last().unwrap());
    
    // Forming the message
    let result = User::new_message(current_user,  data.message);

    // Sending the message to the channel
    Channel::message(current_channel, result);

    HttpResponse::Ok().finish()
}