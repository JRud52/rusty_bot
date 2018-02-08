use serenity::client::bridge::gateway::{ShardManager};
use serenity::model::channel::Message;
use serenity::model::event::ResumedEvent;
use serenity::model::id::ChannelId;
use serenity::model::gateway::Ready;
use serenity::prelude::Mutex;
use serenity::prelude::*;

use typemap::Key;

use std::collections::HashMap;
use std::sync::Arc;
use std::env;

lazy_static! {
    static ref VERSION: String = env::var("CARGO_PKG_VERSION").unwrap();
}

pub struct ShardManagerContainer;

impl Key for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct CommandCounter;

impl Key for CommandCounter {
    type Value = HashMap<String, u64>;
}

pub struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);

        if !(cfg!(debug_assertions)) {
            let bot_channel = ChannelId(395989933539459074);
            let _ = bot_channel.say(format!(
                "This server is now running rusty_bot version {}",
                *VERSION
            ));
        }
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }

    fn message(&self, _: Context, msg: Message) {
        // Mess with clojure bot in server
        if msg.author.id == 394253385886334976
            && msg.content.contains("I see you are talking about Clojure.")
        {
            let _ = msg.channel_id.say(format!(
                "{} that's a cool trick, but have you tried rewriting it in Rust?",
                msg.content.split_whitespace().next().unwrap()
            ));
        }
    }
}