#[macro_use] extern crate log;
#[macro_use] extern crate serenity;

extern crate kankyo;

mod commands;

use serenity::framework::StandardFramework;
use serenity::model::event::ResumedEvent;
use serenity::model::ChannelId;
use serenity::model::Ready;
use serenity::model::Message;
use serenity::prelude::*;
use serenity::http;
use std::collections::HashSet;
use std::env;

use serenity::client::Client;
use serenity::prelude::EventHandler;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

struct Handler;

impl EventHandler for Handler {
    fn on_ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);

        if !(cfg!(debug_assertions)){
            let bot_channel = ChannelId(395989933539459074);
            let _ = bot_channel.say(format!("This server is now running rusty_bot version {}", VERSION));
        }
    }

    fn on_resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }

    fn on_message(&self, _: Context, msg: Message) {
        // Mess with clojure bot in server
        if msg.author.id == 394253385886334976 && msg.content.contains( "I see you are talking about Clojure.") {
            let _ = msg.channel_id.say(format!("{} that's a cool trick, but have you tried rewriting it in Rust?",
                                               msg.content.split_whitespace().next().unwrap()));
        }
    }
}

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("RUSTY_TOKEN").expect("token"), Handler);

    let owners = match http::get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        },
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    client.with_framework(StandardFramework::new()
        .configure(|c| c
            .owners(owners)
            .prefix("~"))

        // Meta commands
        .command("ping", |c| c.exec(commands::meta::ping))
        .command("latency", |c| c.exec(commands::meta::latency))
        .command("version", |c| c.exec(commands::meta::version))
        .command("info", |c| c.exec(commands::meta::info))

        // Owner only commands
        .command("quit", |c| c
            .exec(commands::owner::quit)
            .owners_only(true)));

    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}