use serenity::{client::EventHandler, model::channel::Message, prelude::GatewayIntents, Client};

fn main() {
    let token = "le fromage mystère";

    tokio::spawn(async move {
        let handler = Handler::new();
        Client::builder(token, GatewayIntents::all())
            .event_handler(handler)
            .await
            .expect("invalid config")
            .start()
            .await
            .expect("fatal error");
    });
}

pub struct Handler;

impl Handler {
    pub fn new() -> Self {
        Self
    }
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: serenity::client::Context, _new_message: Message) {
        //
    }
}
