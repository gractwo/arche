use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
};
use serenity::builder::CreateCommand;

pub async fn run(ctx: &Context, command: &CommandInteraction) -> Result<(), serenity::Error> {
    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content("P-p-ping!"),
            ),
        )
        .await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("komenda testowa.")
}
