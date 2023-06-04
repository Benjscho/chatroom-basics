use serde::{Deserialize, Serialize};
use warp::Reply;


#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    user_id: usize
}

#[derive(Serialize, Debug)]
pub struct RegisterResponse {
    url: usize
}

#[derive(Deserialize, Debug)]
pub struct Event {
    topic: String,
    user_id: Option<usize>,
    message: String
}

pub async fn publish_handler(body: Event, clients: Clients) -> Result<impl Reply> {

}
