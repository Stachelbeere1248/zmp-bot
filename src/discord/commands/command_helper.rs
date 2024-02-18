use crate::discord::{Context, Error};
use serenity::all::CreateAllowedMentions;
use std::time::Duration;

pub(crate) async fn send(ctx: Context<'_>, reply: String) -> Result<(), Error> {
    if let Err(why) = ctx
        .send(poise::CreateReply {
            content: Some(reply),
            embeds: vec![],
            attachments: vec![],
            ephemeral: None,
            components: None,
            allowed_mentions: Some(CreateAllowedMentions::new().all_roles(true)),
            reply: false,
            __non_exhaustive: (),
        })
        .await
    {
        println!("Error sending message: {:?}", why)
    }
    Ok(())
}

pub(crate) fn cooldown(ctx: &Context, user: u64, global: u64) -> Option<Result<(), Error>> {
    let mut cooldown_tracker = ctx.command().cooldowns.lock().unwrap();
    let cooldown_durations = poise::CooldownConfig {
        global: Some(Duration::from_secs(global)),
        user: Some(Duration::from_secs(user)),
        guild: None,
        channel: None,
        member: None,
        __non_exhaustive: (),
    };
    match cooldown_tracker.remaining_cooldown((*ctx).cooldown_context(), &cooldown_durations) {
        Some(remaining) => Some(Err(
            format!("Please wait {} seconds", remaining.as_secs()).into()
        )),
        None => {
            cooldown_tracker.start_cooldown((*ctx).cooldown_context());
            None
        }
    }
}
