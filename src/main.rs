use warp::Rejection;

mod client;
mod server;
mod handler;

type Result<T> = std::result::Result<T, Rejection>;

struct Channel {
    uuid: String,
    subscribers: Vec<User>
}

struct User {
    uuid: String
}

struct Message {
    uuid: String,
    content: String
}

fn main() {
    println!("Hello, world!");
}
