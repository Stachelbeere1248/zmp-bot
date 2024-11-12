use serenity::all::{Context, Message};

use crate::error::Error;

pub(crate) async fn on_create(ctx: &Context, msg: &Message) -> Result<(), Error> {
    match msg.guild_id.map(|g| g.get()) {
        None => Ok(()),
        Some(1256217633959841853_u64) => zmp_create(ctx, msg).await,
        _ => Ok(()),
    }
}

async fn zmp_create(ctx: &Context, msg: &Message) -> Result<(), Error> {
    match msg.channel_id.get() {
        1295108216388325386_u64 => {
            msg.react(ctx, '🇼').await?;
            msg.react(ctx, '🇱').await?;
            Ok(())
        }
        _ => Ok(()),
    }
}
