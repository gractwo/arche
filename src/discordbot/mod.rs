use serenity::{
    all::{CreateInteractionResponse, CreateInteractionResponseMessage, Interaction, Ready},
    async_trait,
    prelude::*,
};
use tracing::info;

const TOKEN_ENV: &str = "DISCORD_GRA_MAIN_TOKEN";
const GUILD_ID1: &str = "DISCORD_MAIN_SERVER_ID";
const GUILD_ID2: &str = "DISCORD_RD_SERVER_ID";

struct Handler;

mod commands;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let cmds = vec![commands::ping::register(), commands::kiss::register()];

        let main_guild_id = serenity::model::id::GuildId::from(
            std::env::var(GUILD_ID1).unwrap().parse::<u64>().unwrap(),
        );
        let dev_guild_id = serenity::model::id::GuildId::from(
            std::env::var(GUILD_ID2).unwrap().parse::<u64>().unwrap(),
        );

        match main_guild_id.set_commands(&ctx.http, cmds.clone()).await {
            Ok(_) => info!("Successfully registered commands on main guild.",),
            Err(why) => info!("Failed to register commands on main guild: {why:?}"),
        };
        match dev_guild_id.set_commands(&ctx.http, cmds.clone()).await {
            Ok(_) => info!("Successfully registered commands on dev guild."),
            Err(why) => info!("Failed to register commands on dev guild: {why:?}"),
        };
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let result = match command.data.name.as_str() {
                "ping" => commands::ping::run(&ctx, &command).await,
                "kiss" => commands::kiss::run(&ctx, &command).await,
                _ => {
                    command
                        .create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new().content("Not implemented"),
                            ),
                        )
                        .await
                }
            };

            if let Err(why) = result {
                println!("Error executing command: {:?}", why);
            }
        }
    }
}

pub async fn init() {
    tokio::spawn(async {
        init_bot().await;
    });
}

async fn init_bot() {
    let token = std::env::var(TOKEN_ENV).unwrap();
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("errored creating client");

    if let Err(why) = client.start().await {
        info!("client error: {why:?}");
    }
}
