use requests;
use requests::StatusCode;
use requests::ToJson;

use std::env;

lazy_static! {
    static ref BLIZZARD_API_KEY: String = env::var("BLIZZARD_API_KEY").unwrap();
}

/// Creates a URI that accesses the Blizzard WoW API. Args must be in the format KEY=VALUE
fn wow_uri(endpoint: &'static str, args: Vec<String>) -> String {
    let mut formatted_args: String = String::new();

    for arg in args {
        formatted_args += &format!("&{}", arg);
    }

    return format!(
        "https://us.api.battle.net/wow/{}?locale=en_US{}&apikey={}",
        endpoint, formatted_args, *BLIZZARD_API_KEY
    );
}

command!(realm(_ctx, msg, args) {
    let mut realm_name: String = args.full().to_string();

    if realm_name.is_empty() {
        let _ = msg.channel_id.say(format!("@{} That is not a valid realm name.", msg.author));
    } else {
        realm_name = realm_name.replace(" ", "-");
        let realm_arg: String = format!("realms={}", realm_name);

        let url: String = wow_uri("realm/status", vec!(realm_arg)).parse()?;
        println!("{}", url);

        let res = requests::get(url).unwrap();
        let data = res.json().unwrap();

        if res.status_code() == StatusCode::Ok {
            let _ = msg.channel_id.send_message(|m| m
                .embed(|e| e
                    .title(format!("{}", data["realms"][0]["name"]))
                    .description(
                        format!(    "Status: {}\n\
                                    Type: {}\n\
                                    Pop: {}\n\
                                    Queue: {}\n\
                                    BattleGroup: {}"

                        , data["realms"][0]["status"]
                        , data["realms"][0]["type"]
                        , data["realms"][0]["population"]
                        , data["realms"][0]["queue"]
                        , data["realms"][0]["battlegroup"]))
                ));
        } else {
            let _ = msg.channel_id.say(format!("@{} That is not a valid realm name.", msg.author));
        }
    }
});
