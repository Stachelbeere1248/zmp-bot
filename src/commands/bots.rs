use std::string::String;

use poise::CreateReply;

use crate::error::Error;
use crate::Context;

#[poise::command(
    slash_command,
    owners_only,
    install_context = "User",
    interaction_context = "Guild|BotDm|PrivateChannel",
    ephemeral = "false"
)]
/// Change how many helpstart bots are online, to limit usage of helpstart pings.
pub(crate) async fn bots(
    ctx: Context<'_>,
    #[min = 0_u8]
    #[description = "default: 0"]
    bots: u8,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    *ctx.data().bots.write().await = bots;
    let content = format!("{} bots are now registered as available", bots).to_string();
    ctx.send(CreateReply::default().content(content)).await?;
    Ok(())
}
