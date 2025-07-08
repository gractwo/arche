use crate::discordbot::MAIN_GUILD_ID;
use serenity::all::{Context, GuildId};
use service::run_membercount_service;
use tracing::info;

mod service;

pub use service::get_member_count;

pub fn init_service(ctx: &Context, guild_id: &GuildId) {
    let (ctx, guild_id) = (ctx.clone(), guild_id.clone());
    info!("Initialising discord member count service...");

    if guild_id != MAIN_GUILD_ID {
        info!("Guild member count service not initialised; Bot not running on main guild.");
        return;
    }

    tokio::spawn(async move {
        run_membercount_service(ctx, guild_id).await;
    });
}
