//#[macro_use] extern crate log;
#[macro_use] extern crate serenity;

extern crate requests;
extern crate serde_json;

mod commands;

use serenity::framework::standard::{help_commands, StandardFramework};
use serenity::model::event::ResumedEvent;
use serenity::model::{Ready, ChannelId, Message};
use serenity::client::Client;
use serenity::prelude::*;
use serenity::http;

use std::collections::HashSet;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const RUSTY_TOKEN: &'static str = env!("RUSTY_TOKEN");

struct Handler;

impl EventHandler for Handler {
    fn on_ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);

        if !(cfg!(debug_assertions)){
            let bot_channel = ChannelId(395989933539459074);
            let _ = bot_channel.say(format!("This server is now running rusty_bot version {}", VERSION));
        }
    }

    fn on_resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
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
    let mut client = Client::new(RUSTY_TOKEN, Handler);

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

        .before(|_ctx, msg, command_name| {
            println!("Got command '{}' by user '{}'",
                     command_name,
                     msg.author.name);
            true // if `before` returns false, command processing doesn't happen.
        })
        .after(|_, _, command_name, error| {
            match error {
                Ok(()) => println!("Processed command '{}'", command_name),
                Err(why) => println!("Command '{}' returned error {:?}", command_name, why),
            }
        })

        .group( "Utility", |g| g
            .command("help", |c| c
                .exec_help(help_commands::with_embeds))

            .command("ping", |c| c
                .desc("Responds with 'Pong!'.")
                .exec_str("Pong!"))

            .command("latency", |c| c
                .desc("Responds with the latency of the server.")
                .exec(commands::meta::latency))

            .command("version", |c| c
                .desc("Responds with the version number of the currently running bot.")
                .exec(commands::meta::version))

            .command("info", |c| c
                .desc("Responds with the details of the bot.")
                .exec(commands::meta::info))
        )

        .group( "World of Warcraft", |g| g
            .command("realm", |c| c
                .desc("Responds with the status of the specified realm.")
                .exec(commands::wow::realm))
        )

        .group("Owner only", |g| g
            .owners_only(true)
            .command("quit", |c| c
                .desc("Shuts down the bot(owner only command).")
                .exec(commands::owner::quit))
        )
    );

    if let Err(why) = client.start() {
        panic!("Client error: {:?}", why);
    }
}
