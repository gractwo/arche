use rand::{Rng, SeedableRng, seq::IndexedRandom};
use serenity::all::Context;
use std::time::Duration;
use tokio::time::sleep;
use tracing::info;

use crate::discordbot::status::list::LIST;

mod list;

pub fn init_service(ctx: &Context) {
    info!("Initialising status automation service...");

    let ctx = ctx.clone();
    tokio::spawn(async move {
        run_service(ctx).await;
    });
}

pub async fn run_service(ctx: Context) {
    loop {
        let mut rng = rand::rngs::SmallRng::from_rng(&mut rand::rng());
        let activity = (*LIST).choose(&mut rng).map(|a| a.to_owned());
        ctx.set_activity(activity);

        // sleep for randomly 15, 30 or 45 minutes between status changes
        sleep(Duration::from_secs(60 * 15 * rng.random_range(1..=3))).await;
    }
}
