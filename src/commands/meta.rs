const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

command!(latency(ctx, msg) {
    let latency = ctx.shard.lock()
        .latency()
        .map_or_else(|| "N/A".to_string(), |s| {
            format!("{}.{}s", s.as_secs(), s.subsec_nanos())
        });

    let _ = msg.channel_id.say(latency);
});

command!(ping(_ctx, msg) {
    let _ = msg.channel_id.say("Pong!");
});

command!(version(_ctx, msg) {
    let _ = msg.channel_id;
    let _ = msg.channel_id.say(env!("CARGO_PKG_VERSION"));
});

command!(info(_ctx, msg) {
    let _ = msg.channel_id.say(format!(
    "```yaml\n\
    Name:   {}\n\
    Version:        {}\n\
    Homepage:       '{}'\n\
    Authors:        {}\n\
    Description:    {}\n\
    ```",
    NAME, VERSION, HOMEPAGE, AUTHORS, DESCRIPTION));
});
