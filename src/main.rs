mod bot;
mod store;

use pretty_env_logger::env_logger;


#[tokio::main]
async fn main() {
    env_logger::init();

    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_async_connection().await?;

    bot::start_polling("").await;
}
