use std::time::Duration;
use crate::{Context, Error};

pub(crate) async fn send(
    ctx: Context<'_>,
    reply: String
) -> Result<(), Error> {
    if let Err(why) = ctx
        .send(|m| {
            m.content(reply)
                .allowed_mentions(|am| am.parse(poise::serenity_prelude::ParseValue::Roles))
        })
        .await
    {
        println!("Error sending message: {:?}", why)
    }
    Ok(())
}

pub(crate) fn cooldown(
    ctx: &Context,
    user: u64,
    global: u64
) -> Option<Result<(), Error>> {
    let mut cooldown_tracker = ctx.command().cooldowns.lock().unwrap();
    let mut cooldown_durations = poise::CooldownConfig::default();
    cooldown_durations.user = Some(Duration::from_secs(user));
    cooldown_durations.global = Some(Duration::from_secs(global));

    match cooldown_tracker.remaining_cooldown_2(*ctx, &cooldown_durations) {
        Some(remaining) =>
            Some(Err(format!("Please wait {} seconds", remaining.as_secs()).into())),
        None => {
            cooldown_tracker.start_cooldown(*ctx);
            None
        }
    }
}