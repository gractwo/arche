use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
};
use serenity::builder::CreateCommand;

const PERISH_GIF_LINK: &str = "https://media1.giphy.com/media/v1.Y2lkPTc5MGI3NjExbGM3NDRtaXQ0ZWk3enlybmwydHZmc3VwcWlzbXN1N2Zyc3Jpc2lmYSZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/4kkOaBEy84jzD5ijGV/giphy.gif";

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    interaction
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content(PERISH_GIF_LINK),
            ),
        )
        .await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("perish").description("w proch się obróć.")
}
