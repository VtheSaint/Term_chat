use actix_web::{web::{Json, Data}, Responder};
use actix_web_lab::sse;

use crate::AppState;



pub struct ESData {
    name: String,
    channel_name: String   
}
pub async fn establish_connectioin(
    req: Json<Option<ESData>>,
    state: Data<AppState>
) -> impl Responder {
    let mut response = sse::channel(10).1;
    let data = req.into_inner().unwrap();
    let mut channels = state.channels.lock().unwrap();

    for channel in channels.iter_mut() {
        if channel.name == data.channel_name {
            response = channel.broadcaster.new_client().await;
        }
    }
    response

}