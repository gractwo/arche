use serenity::{
    all::{CreateInteractionResponse, CreateInteractionResponseMessage, Interaction, Ready},
    async_trait,
    prelude::*,
};
use tracing::info;

const TOKEN_ENV: &str = "DISCORD_BOT_TOKEN";
const GUILD_ID: &str = "DISCORD_SERVER_ID";

struct Handler;

mod commands;
mod guild_icon;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let cmds = vec![
            commands::ping::register(),
            commands::kiss::register(),
            commands::perish::register(),
        ];

        let guild_id = serenity::model::id::GuildId::from(
            std::env::var(GUILD_ID).unwrap().parse::<u64>().unwrap(),
        );

        match guild_id.set_commands(&ctx.http, cmds.clone()).await {
            Ok(_) => info!("Successfully registered commands on the guild.",),
            Err(why) => info!("Failed to register commands on the guild: {why:?}"),
        };

        guild_icon::init_service(&ctx, &guild_id);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(cmd) = interaction {
            let result = match cmd.data.name.as_str() {
                "ping" => commands::ping::run(&ctx, &cmd).await,
                "kiss" => commands::kiss::run(&ctx, &cmd).await,
                "perish" => commands::perish::run(&ctx, &cmd).await,
                _ => {
                    cmd.create_response(
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
    let intents = GatewayIntents::non_privileged();

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("errored creating client");

    if let Err(why) = client.start().await {
        info!("client error: {why:?}");
    }
}
