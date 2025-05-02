use serenity::all::{
    Colour, CommandInteraction, CommandOptionType, Context, CreateCommandOption, CreateEmbed,
    CreateEmbedAuthor, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage,
    EditInteractionResponse, Permissions, ResolvedValue,
};
use serenity::builder::CreateCommand;

const GRACTWO_X64_GHLINK: &str =
    "https://raw.githubusercontent.com/gractwo/assets/refs/heads/master/raster/logo-x64.png";

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let options = interaction.data.options();

    let embed_type = options
        .iter()
        .find_map(|opt| match (opt.name, &opt.value) {
            ("type", ResolvedValue::String(str)) => Some(*str),
            _ => None,
        })
        .expect("Embed type is a required field; should be present.");

    let embed = match embed_type {
        "infowitaj" => Some(CreateEmbed::new()
            .author(CreateEmbedAuthor::new("Witajcie w Gractwie!").icon_url(GRACTWO_X64_GHLINK))
            .description("Jesteśmy grupą ludzi których kręcą gry. Proste, nie?\nSerwer ten miał wcześniej tematykę Team Fortressową.")
            .colour(Colour::BLITZ_BLUE)
            .field("Zasady i regulamin", "Zgrubsza: nie bądź uschłą wierzbą.", false)
            .field("Gractwo WWW", "https://gractwo.pl", true)
            .field("Gractwo DISCORD", "https://gractwo.pl/discord", true)
            .field("DISCORD bezpośredni", "discord.gg/NBXq95C", true)
            .field("Gractwo BLUESKY", "https://gractwo.pl/bsky", true)
            .field("Gractwo YOUTUBE", "https://gractwo.pl/youtube", true)
            .field("Gractwo GITHUB", "https://gractwo.pl/github", true)),
        _ => None
    };

    let message = CreateInteractionResponseMessage::new()
        .ephemeral(true)
        .content(if embed.is_some() {
            format!("Sending embed of type {embed_type}...")
        } else {
            format!("{embed_type} isn't a valid embed type")
        });
    interaction
        .create_response(&ctx.http, CreateInteractionResponse::Message(message))
        .await
        .ok();

    match embed {
        Some(e) => {
            let msg = CreateMessage::new().embed(e);
            match interaction.channel_id.send_message(&ctx.http, msg).await {
                Ok(_) => interaction
                    .edit_response(
                        &ctx.http,
                        EditInteractionResponse::new().content("Success!"),
                    )
                    .await
                    .map(|_| ()),
                Err(why) => interaction
                    .edit_response(
                        &ctx.http,
                        EditInteractionResponse::new()
                            .content(format!("Failed to send embed: {why}")),
                    )
                    .await
                    .map(|_| ()),
            }
        }
        None => Ok(()),
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("embed")
        .description("komenda do wysyłania embedów.")
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "type", "type").required(true),
        )
}
