use crate::discord::commands::{command_helper, helpstart};
use crate::discord::{Context, Error};
use std::string::String;

#[poise::command(slash_command, guild_only)]
pub(crate) async fn bots(
    ctx: Context<'_>,

    #[min = 0_u8]
    #[description = "default: 0"]
    bots: u8,
) -> Result<(), Error> {
    helpstart::set_bots_available(bots).await;

    let reply = format!("{} bots are now registered as available", bots).to_string();
    command_helper::send(ctx, reply).await
}
