use actix_web::{web::{Json, Data}, HttpResponse};
use uuid::Uuid;

use crate::{AppState, models::{user::User, channel::Channel}};



pub struct DropData {
    user_id: Uuid,
    channel_id: Uuid,
}

pub fn drop_user(
    req: Json<Option<DropData>>,
    state: Data<AppState>
) -> HttpResponse {
    let data = req.into_inner().unwrap();
    let mut channels = state.channels.lock().unwrap();
    for channel in channels.iter_mut() {
        if channel.id == data.channel_id {
            // let result = User::leave_channel(self, group);\
            for user in channel.clone().users.iter_mut() {
                if user.id == data.user_id {
                    
                    // Construct message
                    let result = User::leave_channel(&user, channel.name.as_str());
                    
                    // Remove user from channel
                    Channel::remove_user(channel, &user);

                    // Publish message to channel
                    Channel::message(&channel, result);
                    break;
                }
            }

        }
    }


    HttpResponse::Ok().finish()

}