//#[macro_use] extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serenity;

extern crate requests;
extern crate typemap;

mod commands;
mod shared;

use serenity::framework::standard::{DispatchError, StandardFramework, HelpBehaviour, help_commands};
use serenity::http;
use serenity::prelude::*;

use std::collections::HashMap;
use std::sync::Arc;

use std::collections::HashSet;
use std::env;

use shared::shared_structs::Handler;
use shared::shared_structs::CommandCounter;
use shared::shared_structs::ShardManagerContainer;

lazy_static! {
    static ref RUSTY_TOKEN: String = env::var("RUSTY_TOKEN").unwrap();
}

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&RUSTY_TOKEN, Handler).expect("Err creating client");

    {
        let mut data = client.data.lock();
        data.insert::<CommandCounter>(HashMap::default());
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    let owners = match http::get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        }
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.owners(owners).prefix("~"))
            .before(|ctx, msg, command_name| {
                println!(
                    "Got command '{}' by user '{}'",
                    command_name, msg.author.name
                );

                let mut data = ctx.data.lock();
                let counter = data.get_mut::<CommandCounter>().unwrap();
                let entry = counter.entry(command_name.to_string()).or_insert(0);
                *entry += 1;

                true // if `before` returns false, command processing doesn't happen.
            })
            .after(|_, _, command_name, error| match error {
                Ok(()) => println!("Processed command '{}'", command_name),
                Err(why) => println!("Command '{}' returned error {:?}", command_name, why),
            })
            .on_dispatch_error(|_ctx, msg, error| {
                if let DispatchError::RateLimited(seconds) = error {
                    let _ = msg.channel_id.say(&format!("Try this again in {} seconds.", seconds));
                }
            })
            .customised_help(help_commands::with_embeds, |c| {
                c.individual_command_tip("Hello! \n\
                    If you want more information about a specific command, just pass the command as argument.")
                .command_not_found_text("Could not {}, I'm sorry : (")
                .suggestion_text("How about this command: {}")
                .lacking_permissions(HelpBehaviour::Hide)
                .lacking_role(HelpBehaviour::Nothing)
                .wrong_channel(HelpBehaviour::Strike)
            })

            .group("Utility", |g| {
                g.command("latency", |c| {
                    c.desc("Responds with the latency of the server.")
                        .cmd(commands::meta::latency)
                })
                .command("version", |c| {
                    c.desc("Responds with the version number of the currently running bot.")
                        .cmd(commands::meta::version)
                })
                .command("info", |c| {
                    c.desc("Responds with the details of the bot.")
                        .cmd(commands::meta::info)
                })
            })
            .group("World of Warcraft", |g| {
                g.command("realm", |c| {
                    c.desc("Responds with the status of the specified realm.")
                        .cmd(commands::wow::realm)
                })
            })
            // .group("Owner only", |g| {
            //     g.owners_only(true).command("quit", |c| {
            //         c.desc("Shuts down the bot(owner only command).")
            //             .exec(commands::owner::quit)
            //     })
            // }),
    );

    if let Err(why) = client.start() {
        panic!("Client error: {:?}", why);
    }
}
