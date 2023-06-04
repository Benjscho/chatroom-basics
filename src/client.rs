use std::{collections::HashMap, sync::Arc};

use tokio::sync::{mpsc, RwLock};
use warp::ws::Message;

#[derive(Debug, Clone)]
pub struct Client {
    pub user_id: usize,
    pub topics: Vec<String>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>
}

type Clients = Arc<RwLock<HashMap<String, Client>>>;

