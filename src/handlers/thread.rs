use serenity::all::{Context, GuildChannel};
use serenity::builder::EditThread;

use crate::error::Error;

pub(crate) async fn on_create(ctx: &Context, thread: &GuildChannel) -> Result<(), Error> {
    match thread.parent_id.map(|parent| parent.get()) {
        Some(1295108216388325386) => {
            thread.id.edit_thread(ctx, EditThread::new().rate_limit_per_user(7200_u16)).await?;
            Ok(())
        }
        Some(_) => Ok(()),
        None => Ok(())
    }
}