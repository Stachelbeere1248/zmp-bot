use std::string::String;
use poise::CreateReply;
use crate::Context;
use crate::error::Error;

#[poise::command(slash_command, guild_only, owners_only)]
pub(crate) async fn bots(
    ctx: Context<'_>,
    #[min = 0_u8]
    #[description = "default: 0"]
    bots: u8,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    *ctx.data().bots.write().await = bots;
    let content = format!("{} bots are now registered as available", bots).to_string();
    ctx.send(CreateReply::default().content(content).ephemeral(true)).await?;
    Ok(())
}
