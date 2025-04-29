use serenity::all::{
    CommandInteraction, Context, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};
use serenity::builder::CreateCommand;

const PERISH_GIF_LINK: &str = "https://media1.giphy.com/media/v1.Y2lkPTc5MGI3NjExbGM3NDRtaXQ0ZWk3enlybmwydHZmc3VwcWlzbXN1N2Zyc3Jpc2lmYSZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/4kkOaBEy84jzD5ijGV/giphy.gif";

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let caller_name = interaction
        .member
        .as_ref()
        .map(|m| m.nick.clone().unwrap_or_else(|| m.user.name.clone()))
        .unwrap_or_else(|| interaction.user.name.clone());

    let embed = CreateEmbed::new()
        .thumbnail(PERISH_GIF_LINK)
        .title(format!("{caller_name} has perished!"))
        .description("Oh the misery...")
        .color(0xFF89B4);

    interaction
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().add_embed(embed),
            ),
        )
        .await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("perish").description("w proch się obróć.")
}
