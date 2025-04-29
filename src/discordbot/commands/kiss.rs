use rand::seq::IndexedRandom;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommandOption, CreateEmbed,
    CreateInteractionResponse, CreateInteractionResponseMessage, MessageBuilder, ResolvedValue,
};
use serenity::builder::CreateCommand;

const KISS_GIF_LINK: &str = "https://media.discordapp.net/attachments/594222795999805643/1078363884190183485/ezgif-1-96508f9a03.gif?ex=681108a1&is=680fb721&hm=719ea63308d9f2989fbdc8110735805c2e9c26a378e749785fa141ddb180203f";

const KISS_TEXTS: &[&str] = &[
    "How endearing... :3 :3",
    "What does this imply!?",
    "What a cutie!",
    "That's a little gay...",
    "With tongue though?!",
    "Will they reciprocate?",
    "And they were roommates.",
    "How brave!",
    "No homo.",
    "Better than \"kys\".",
    "What comes next?",
];

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let options = &interaction.data.options();

    let target_user = options
        .iter()
        .find_map(|opt| match (opt.name, &opt.value) {
            ("target", ResolvedValue::User(user, member)) => Some((user, member.as_ref())),
            _ => None,
        })
        .expect("Target user is a required field; should be present");

    let use_bigger = options
        .iter()
        .find_map(|opt| match (opt.name, &opt.value) {
            ("target", ResolvedValue::Boolean(v)) => Some(*v),
            _ => None,
        })
        .unwrap_or(false);

    let caller_name = interaction
        .member
        .as_ref()
        .map(|m| m.nick.clone().unwrap_or_else(|| m.user.name.clone()))
        .unwrap_or_else(|| interaction.user.name.clone());

    let target_name = match target_user {
        (user, Some(member)) => member.nick.clone().unwrap_or_else(|| user.name.clone()),
        (user, None) => user.name.clone(),
    };

    let rand_text = KISS_TEXTS.choose(&mut rand::rng()).unwrap();
    let mention = MessageBuilder::new()
        .mention(target_user.0.to_owned())
        .build();
    let mut embed = CreateEmbed::new()
        .title(format!("{} kisses {}", caller_name, target_name))
        .description(format!("{rand_text}\n{mention}"))
        .color(0xFF89B4);

    if use_bigger {
        embed = embed.image(KISS_GIF_LINK.to_string())
    } else {
        embed = embed.thumbnail(KISS_GIF_LINK.to_string())
    }

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
    CreateCommand::new("kiss")
        .description("pocałuj kogoś.")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::User,
                "target",
                "użytkownik do pocałowania",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Boolean,
                "bigger",
                "czy użyć wersji z większym obrazkiem?",
            )
            .required(false),
        )
}
