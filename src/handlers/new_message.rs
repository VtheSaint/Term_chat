use actix_web::{web::{Json, Data}, HttpResponse};
use serde::Deserialize;

use crate::{models::{user::User, channel::Channel}, AppState};

#[derive(Deserialize, Debug)]
pub struct MessageData {
    pub user_name: String,
    pub channel_name: String,
    pub message: String,
}


pub async fn new_message(
    req: Json<Option<MessageData>>,
    state: Data<AppState>
) -> HttpResponse {

    
    // Get data from request
    let data = req.into_inner().unwrap();
    
    // get users and channels from state
    let users = state.users.lock().unwrap();
    let channels = state.channels.lock().unwrap();
    
    
    // println!("Users: {:#?}", users);
    // println!("Data: {:#?}", data);


    // Get current user and channel from Vecs 
    let current_user = *(users.iter().filter(|u| u.name == data.user_name).collect::<Vec<&User>>().last().unwrap());
    let current_channel = *(channels.iter().filter(|c| c.name == data.channel_name).collect::<Vec<&Channel>>().last().unwrap());
    
    // Forming the message
    let result = User::new_message(current_user,  data.message);

    // Sending the message to the channel
    Channel::message(current_channel, result).await;

    HttpResponse::Ok().finish()
}