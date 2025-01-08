use std::time::Duration;

use crate::error::Error;
use crate::Context;

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
    if ctx.framework().options.owners.contains(&ctx.author().id) {
        Ok(())
    } else {
        match cooldown_tracker.remaining_cooldown((*ctx).cooldown_context(), &cooldown_durations) {
            Some(remaining) => Err(Error::OnCooldown(remaining)),
            None => Ok(cooldown_tracker.start_cooldown((*ctx).cooldown_context())),
        }
    }
}
