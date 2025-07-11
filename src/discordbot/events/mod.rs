use crate::discordbot::MAIN_GUILD_ID;
use chrono::{Datelike, Utc};
use chrono_tz::Europe::Warsaw;
use rand::{SeedableRng, seq::IndexedRandom};
use serenity::all::{Context, GuildId};
use service::run_event_service;
use tracing::info;

mod service;

pub fn init_service(ctx: &Context, guild_id: &GuildId) {
    let (ctx, guild_id) = (ctx.clone(), guild_id.clone());
    info!("Initialising event automation service...");

    if guild_id != MAIN_GUILD_ID {
        info!("Guild event automation service not initialised; Bot not running on main guild.");
        return;
    }

    tokio::spawn(async move {
        run_event_service(ctx, guild_id).await;
    });
}

pub enum Event {
    Normal,
    PolskaGórą,
    PrideMonth,
    RandomQueerEvent,
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
    pub fn get_current() -> Self {
        let now = Utc::now().with_timezone(&Warsaw);
        match (now.day(), now.month()) {
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
            _ => match rand::random_bool(1.0 / 24.0) {
                true => Event::RandomQueerEvent,
                false => Event::Normal,
            },
        }
    }
    pub fn icon(&self) -> &str {
        use Event as E;
        match self {
            E::PolskaGórą => "./assets/logo-x512-polish.png",
            E::PrideMonth | E::RandomQueerEvent => "./assets/logo-x512-lgbtflag.png",
            E::StarWarsDay => "./assets/logo-x512-starwars.png",
            _ => "./assets/logo-x512.png",
        }
    }
    pub fn guild_name(&self) -> String {
        use Event as E;
        match self {
            E::PolskaGórą => {
                let now = Utc::now().with_timezone(&Warsaw);
                let suffix = match (now.day(), now.month()) {
                    (1, 5) => ": Święto Pracy",
                    (2, 5) => ": Dzień Flagi",
                    (3, 5) => ": Święto Konstytucji",
                    _ => "",
                };
                format!("Gractwo{suffix}")
            }
            E::PrideMonth => {
                let now = Utc::now().with_timezone(&Warsaw);
                let seed: u64 = (now.year() as u64) * 1000 + (now.day() as u64);
                let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
                const VARIANTS: &[&str] = &[
                    "Miesiąc Dumy",
                    "Swoboda!",
                    "Równość!",
                    "Pride Month",
                    "LGBT Rights!",
                    "Rainbows Galore!",
                    "Rainbow Barrage!",
                    "Rainbow Resistance",
                    "Resist!",
                    "All Are Welcome",
                    "USB-C LGTV!",
                    "Unite!",
                    "Proud!",
                ];
                format!("Gractwo: {}", VARIANTS.choose(&mut rng).unwrap())
            }
            E::RandomQueerEvent => "Gractwo: Random Queer Event".to_string(),
            E::ValentineDay => {
                let mut rng = rand::rng();
                const VARIANTS: &[&str] = &["Gractwo is in love!", "Gractwo Dating Sim"];
                VARIANTS.choose(&mut rng).unwrap().to_string()
            }
            E::Rogaliki => "Rogalictwo 🥐".to_string(),
            E::Halloween => "Spooky Gractwo 🎃".to_string(),
            E::Christmas => "Jolly Gractwo 🎄🎁☃️".to_string(),
            E::NewYears => "New Gractwo, New Me 🎉".to_string(),
            E::AnniversaryTF2 => "PGTF: Polscy Gracze Team Fortress".to_string(),
            E::AnniversaryGractwo => {
                // TODO: this will conjugate badly in 17 years
                let xlecie = Utc::now().year_ce().1 - 2020;
                format!("{xlecie} lat Gractwa!")
            }
            _ => "Gractwo".to_string(),
        }
    }
}
