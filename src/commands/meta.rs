use serenity::client::bridge::gateway::{ShardId};

use shared::shared_structs::ShardManagerContainer;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

command!(latency(ctx, msg, _args) {
    let data = ctx.data.lock();

    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(v) => v,
        None => {
            let _ = msg.reply("There was a problem getting the shard manager");

            return Ok(());
        },
    };

    let manager = shard_manager.lock();
    let runners = manager.runners.lock();

    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            let _ = msg.reply("No shard found");

            return Ok(());
        },
    };

    let _ = msg.reply(&format!("The shard latency is {:?}", runner.latency));
});

command!(version(_ctx, msg) {
    let _ = msg.channel_id;
    let _ = msg.channel_id.say(format!("This server is running rusty_bot version {}", env!("CARGO_PKG_VERSION")));
});

command!(info(_ctx, msg) {
    let _ = msg.channel_id.send_message(|m| m
                .embed(|e| e
                    .description(
                        format!(
                            "
                            **Name**:            {}\n\
                            **Version**:         {}\n\
                            **Homepage**:   {}\n\
                            **Authors**:        {}\n\
                            **Description**:  {}\n\
                            ",
                            NAME, VERSION, HOMEPAGE, AUTHORS, DESCRIPTION
                        )
                    )
                ));
});
