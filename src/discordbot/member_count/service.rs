use serenity::all::{Context, GuildId};
use std::{
    sync::{Arc, LazyLock},
    time::Duration,
};
use tokio::{sync::RwLock, time::sleep};
use tracing::error;

static MEMBER_COUNT: LazyLock<Arc<RwLock<Option<u64>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(None)));

pub async fn run_membercount_service(ctx: Context, guild_id: GuildId) {
    loop {
        update_member_count(&ctx, guild_id).await;
        sleep(Duration::from_secs(60)).await;
    }
}

pub async fn get_member_count() -> Option<u64> {
    *MEMBER_COUNT.read().await
}

async fn update_member_count(ctx: &Context, guild_id: GuildId) {
    let count = match ctx.http.get_guild_with_counts(guild_id).await {
        Ok(guild) => guild.approximate_member_count,
        Err(e) => {
            error!("Could not fetch guild member count: {e}");
            None
        }
    };
    let mut member_count = MEMBER_COUNT.write().await;
    *member_count = count;
}
