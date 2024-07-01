use std::time::Duration;

use crate::{Context, Error};

pub(crate) async fn send_simple(ctx: Context<'_>, reply: String) -> Result<(), Error> {
    if let Err(why) = ctx
        .send(poise::CreateReply {
            content: Some(reply),
            embeds: vec![],
            attachments: vec![],
            ephemeral: Some(true),
            components: None,
            allowed_mentions: None,
            reply: false,
            __non_exhaustive: (),
        })
        .await
    {
        println!("Error sending message: {:?}", why)
    }
    Ok(())
}

pub(crate) fn cooldown(ctx: &Context, user: u64, guild: u64) -> Result<(), Error> {
    let mut cooldown_tracker = ctx.command().cooldowns.lock().unwrap();
    let cooldown_durations = poise::CooldownConfig {
        global: None,
        user: Some(Duration::from_secs(user)),
        guild: Some(Duration::from_secs(guild)),
        channel: None,
        member: None,
        __non_exhaustive: (),
    };
    match cooldown_tracker.remaining_cooldown((*ctx).cooldown_context(), &cooldown_durations) {
        Some(remaining) => Err(format!("Please wait {} seconds", remaining.as_secs()).into()),
        None => Ok(cooldown_tracker.start_cooldown((*ctx).cooldown_context())),
    }
}
