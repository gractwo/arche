use chrono::{Datelike, Duration, Local, TimeZone, Utc};
use chrono_tz::Europe::Warsaw;
use serenity::all::{Context, CreateAttachment, EditGuild, Guild, GuildId};
use std::time::Duration as StdDuration;
use tracing::{error, info, warn};

const MAIN_GUILD_ID: GuildId = GuildId::new(447075692664979466);

enum Event {
    Normal,
    PolskaGórą,
    PrideMonth,
    ValentineDay,
    Rogaliki,
    Halloween,
    Christmas,
    NewYears,
    AnniversaryGractwo,
    AnniversaryTF2,
    AnniversaryMinecraft,
    StarWarsDay,
}

impl Event {
    fn icon(&self) -> &str {
        use Event as E;
        match self {
            E::PolskaGórą => "./assets/logo-x512-polish.png",
            E::PrideMonth => "./assets/logo-x512-lgbtflag.png",
            E::StarWarsDay => "./assets/logo-x512-starwars.png",
            _ => "./assets/logo-x512.png",
        }
    }
}

fn get_current_event() -> Event {
    let today = Local::now();
    let (month, day) = (today.month(), today.day());

    match (day, month) {
        (1, 1) => Event::NewYears,
        (2..=6, 1) => Event::Christmas,
        (14, 2) => Event::ValentineDay,
        (1..=3, 5) => Event::PolskaGórą,
        (4, 5) => Event::StarWarsDay,
        (17, 5) => Event::AnniversaryMinecraft,
        (7, 6) => Event::AnniversaryGractwo,
        (_, 6) => Event::PrideMonth,
        (24, 8) => Event::AnniversaryTF2,
        (10, 10) => Event::AnniversaryTF2,
        (31, 10) => Event::Halloween,
        (11, 11) => Event::Rogaliki,
        (24..=30, 12) => Event::Christmas,
        (31, 12) => Event::NewYears,
        _ => Event::Normal,
    }
}

pub fn init_service(ctx: &Context, guild_id: &GuildId) {
    let (ctx, guild_id) = (ctx.clone(), guild_id.clone());
    info!("Initialising guild name/icon automation service...");

    if guild_id != MAIN_GUILD_ID {
        info!("Guild name/icon automation service not initialised; Bot not running on main guild.");
        return;
    }

    tokio::spawn(async move {
        run_icon_service(ctx, guild_id).await;
    });
}

pub async fn run_icon_service(ctx: Context, guild_id: GuildId) {
    loop {
        update_icon(&ctx, guild_id).await.ok();
        sleep_until_next_midnight().await;
    }
}

async fn update_icon(ctx: &Context, guild_id: GuildId) -> Result<(), String> {
    let mut guild = match Guild::get(&ctx.http, guild_id).await {
        Ok(g) => g,
        Err(e) => {
            error!("Could not get guild info...");
            return Err("Could not get guild info: {e}".to_string());
        }
    };
    let event = get_current_event();

    let icon = match CreateAttachment::path(event.icon()).await {
        Ok(i) => i,
        Err(e) => {
            error!("Could not create icon attachment...");
            return Err("Could not create icon attachment: {e}".to_string());
        }
    };
    match guild
        .edit(&ctx.http, EditGuild::new().icon(Some(&icon)))
        .await
    {
        Ok(_) => info!("Guild icon updated."),
        Err(e) => {
            error!("Could not update guild icon...");
            return Err("Could not update guild icon: {e}".to_string());
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
            StdDuration::from_secs(1 * 60 * 60)
        }
    };

    tokio::time::sleep(sleeptime).await;
}
