use std::string::String;

use crate::commands::command_helper;
use crate::{Context, Error};

#[poise::command(slash_command, guild_only, owners_only)]
pub(crate) async fn bots(
    ctx: Context<'_>,
    #[min = 0_u8]
    #[description = "default: 0"]
    bots: u8,
) -> Result<(), Error> {
    *ctx.data().bots.write().await = bots;
    let reply = format!("{} bots are now registered as available", bots).to_string();
    command_helper::send_simple(ctx, reply).await
}
