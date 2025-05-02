use chrono::{Datelike, Duration, TimeZone, Utc};
use chrono_tz::Europe::Warsaw;
use serenity::all::{Context, CreateAttachment, EditGuild, Guild, GuildId};
use tracing::{error, info, warn};

use super::Event;

pub async fn run_event_service(ctx: Context, guild_id: GuildId) {
    loop {
        update_guild(&ctx, guild_id).await.ok();
        sleep_until_next_midnight().await;
    }
}

async fn update_guild(ctx: &Context, guild_id: GuildId) -> Result<(), String> {
    let mut guild = match Guild::get(&ctx.http, guild_id).await {
        Ok(g) => g,
        Err(e) => {
            error!("Could not get guild info: {e}");
            return Err("Could not get guild info: {e}".to_string());
        }
    };
    let event = Event::get_current();

    let icon = match CreateAttachment::path(event.icon()).await {
        Ok(i) => i,
        Err(e) => {
            error!("Could not create icon attachment: {e}");
            return Err("Could not create icon attachment: {e}".to_string());
        }
    };

    match guild
        .edit(
            &ctx.http,
            EditGuild::new().name(event.guild_name()).icon(Some(&icon)),
        )
        .await
    {
        Ok(_) => info!("Guild name/icon updated."),
        Err(e) => {
            error!("Could not update guild name/icon: {e}");
            return Err("Could not update guild name/icon: {e}".to_string());
        }
    };

    Ok(())
}

async fn sleep_until_next_midnight() {
    let now = Utc::now().with_timezone(&Warsaw);
    let tomorrow = now.date_naive() + chrono::Duration::days(1);

    let next_midnight = match Warsaw
        .with_ymd_and_hms(
            tomorrow.year_ce().1 as i32,
            tomorrow.month(),
            tomorrow.day(),
            0,
            0,
            0,
        )
        .earliest()
    {
        Some(nm) => nm,
        None => Utc::now().with_timezone(&Warsaw) + Duration::minutes(61),
        // e.g. jumps in time occur at 2AM/3AM, Midnight shouldn't be affected
        // but even if it was, returning None, a gap shouldn't last more than an hour.
    };

    let sleeptime = match next_midnight.signed_duration_since(now).to_std() {
        Ok(t) => t,
        Err(_) => {
            warn!("The next midnight is, supposedly, not in the future.");
            warn!("Sleeping an hour instead...");
            std::time::Duration::from_secs(1 * 60 * 60)
        }
    };

    tokio::time::sleep(sleeptime).await;
}
